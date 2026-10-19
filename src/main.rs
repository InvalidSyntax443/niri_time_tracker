use std::{fs, io::Error, io::ErrorKind, path::PathBuf};
use directories::ProjectDirs;
use niri_ipc::{Event, Response, socket::Socket};
use rusqlite::Connection;
use tokio::sync::mpsc;
use crate::{app_actor::{AppActor, WindowCommand}, app_actor::App};


const APP_NAME: &str = "niri_tracker";


 #[tokio::main]
 async fn main() -> Result<(), Error>{
    let home_directory = get_storage_dir()?;
    let conn = Connection::open(home_directory.join("AppsBD.db")).expect("Failed to create the database");

    conn.execute(
        "CREATE TABLE Apps (
            app_id  VARCHAR(50),
            activity_date TEXT NOT NULL DEFAULT CURRENT_DATE,
            activity_time INTEGER,
            PRIMARY KEY (app_id, activity_date)
            )",
        (), 
    ).unwrap_or(0);

    let (tx, rx) = mpsc::channel::<WindowCommand>(100);
    let app_actor = AppActor::new(rx);

    tokio::spawn( app_actor.run(home_directory));

    let mut socket_event = Socket::connect()?;
    let mut socket_window = Socket::connect()?;

    socket_event.send(niri_ipc::Request::EventStream)?.unwrap();
    let mut read_event = socket_event.read_events();
    
    loop {
         if let Ok(event) = read_event(){
             match event{
                 Event::WindowFocusChanged { id: _ } => {
                     let response = socket_window.send(niri_ipc::Request::FocusedWindow)?.unwrap();
                     let Response::FocusedWindow(window) = response else { panic!("unexpected response: expected FocusedWindow") };
                     if let Some(window) = window{
                        let app = App::new(window);
                        if tx.send(WindowCommand::Open(app)).await.is_err(){
                            eprintln!("Sender error")
                        };
                     } else {
                        if tx.send(WindowCommand::Close).await.is_err(){
                            eprintln!("Sender error")
                        };
                     }
                 }, 
                 Event::WindowOpenedOrChanged { window } => {
                    let app = App::new(window);
                    if tx.send(WindowCommand::Open(app)).await.is_err(){
                        eprintln!("Sender error")
                    };
                 },
                 _ => {},
             }
         } 
     }
 }

fn get_storage_dir() -> Result<PathBuf, Error>{
    if let Some(home_dir) = ProjectDirs::from("", "", APP_NAME){
        let data_dir = home_dir.data_dir().to_path_buf();
        fs::create_dir_all(&data_dir)?;
        Ok(data_dir)
    } else {
        return Err(Error::new(ErrorKind::NotFound, "Could not determine the user's home directory"));
    }
}

mod app_actor;