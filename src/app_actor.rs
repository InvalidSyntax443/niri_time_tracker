use std::{path::PathBuf, time::{SystemTime, UNIX_EPOCH}};
use rusqlite::Connection;
use tokio::sync::mpsc;
use niri_ipc::Window;

pub enum WindowCommand{
    Open(App),
    Close,
}

pub struct AppActor{
    receiver: mpsc::Receiver<WindowCommand>,
    current_window: Option<App>,
    previous_window: Option<App>,
}

impl AppActor {
    pub fn new(rx: mpsc::Receiver<WindowCommand>) -> Self{
        Self { receiver: rx, current_window: None, previous_window: None }
    }

    pub async fn run(mut self, home_directory: PathBuf){
        while let Some(cmd) = self.receiver.recv().await {
            self.previous_window = self.current_window;

            match cmd{
                WindowCommand::Open(app) => {
                    self.current_window = Some(app);
                },
                WindowCommand::Close => {
                    self.current_window = None;
                }
            }

            if let Err(e) = self.write_bd(&home_directory).await{
                eprintln!("{}", e);
            }
        }
    }

    async fn write_bd(&self, home_directory: &PathBuf) -> Result<(), rusqlite::Error>{
        let now_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time has gone into reverse.").as_secs();
        let conn = Connection::open(home_directory.join("AppsBD.db"))?;


        if let Some(ref current) = self.current_window{
            conn.execute(
                "INSERT OR IGNORE INTO Apps (app_id, activity_time) VALUES (?1, ?2);",
                (&current.app_id, 0))?;
        }

        if let Some(ref previous ) = self.previous_window{
            let time = now_time - previous.focus_time;
            conn.execute(
                "UPDATE Apps SET activity_time = activity_time + ?1 WHERE app_id = ?2",
                (time as i32, &previous.app_id))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct App{
    pub app_id: String,
    pub focus_time: u64,
}

impl App {
    pub fn new(window: Window) -> Self{
        let app_id = window.app_id.unwrap_or(String::from("Unknown"));
        let now_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time has gone into reverse.").as_secs();

        Self { app_id, focus_time: now_time }
    }
}
