use std::time::{SystemTime, UNIX_EPOCH};
use niri_ipc::Window;

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