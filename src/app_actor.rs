use niri_ipc::Window;
use rusqlite::Connection;
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::mpsc;

pub struct AppActor {
    receiver: mpsc::Receiver<Option<App>>,
    current_window: Option<App>,
    previous_window: Option<App>,
}

impl AppActor {
    pub fn new(rx: mpsc::Receiver<Option<App>>) -> Self {
        Self {
            receiver: rx,
            current_window: None,
            previous_window: None,
        }
    }

    pub async fn run(mut self, home_directory: PathBuf) {
        while let Some(app) = self.receiver.recv().await {
            self.previous_window = self.current_window;
            self.current_window = app;

            if let Err(e) = self.write_bd(&home_directory).await {
                eprintln!("{}", e);
            }
        }
    }

    async fn write_bd(&self, home_directory: &PathBuf) -> Result<(), rusqlite::Error> {
        let now_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time has gone into reverse.")
            .as_secs();
        let conn = Connection::open(home_directory)?;

        if let Some(ref current) = self.current_window {
            conn.execute(
                "INSERT OR IGNORE INTO Apps (app_id, activity_time, activity_date) VALUES (?1, ?2, date('now', 'localtime'));",
                (&current.app_id, 0))?;
        }

        if let Some(ref previous) = self.previous_window {
            let time = now_time - previous.focus_time;
            conn.execute(
                "UPDATE Apps SET activity_time = activity_time + ?1 WHERE app_id = ?2",
                (time as i32, &previous.app_id),
            )?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct App {
    pub app_id: String,
    pub focus_time: u64,
}

impl App {
    pub fn new(window: Window) -> Option<Self> {
        let now_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|err| {
                eprintln!("Time error: {err}");
                None::<App>
            })
            .ok()?
            .as_secs();

        if let Some(app_id) = window.app_id {
            return Some(Self {
                app_id,
                focus_time: now_time,
            });
        }

        eprintln!("Could not recognize the app id, pid - {:?}", window.pid);
        None
    }
}
