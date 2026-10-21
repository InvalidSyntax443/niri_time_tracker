use std::{fs, io::{Error, ErrorKind}, path::PathBuf};
use clap::Parser;
use directories::ProjectDirs;


const APP_NAME: &str = "niri_tracker";


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    path: Option<PathBuf>,
    #[arg(short, long)]
    file_name: Option<String>,
}

pub fn get_storage_dir() -> Result<PathBuf, Error>{
    let user_path = Cli::parse().path;
    let file_name = Cli::parse().file_name.unwrap_or("AppsBD.db".to_string());

    match user_path{
        Some(path) => {
            if let Some(parent) = path.parent(){
                fs::create_dir_all(parent)?;
            }

            if path.is_dir(){
                Ok(path.join(file_name))
            } else{
                Ok(path)
            }
        },
        None => {
            if let Some(home_dir) = ProjectDirs::from("", "", APP_NAME){
                let data_dir = home_dir.data_dir().to_path_buf();
                fs::create_dir_all(&data_dir)?;
                Ok(data_dir.join(file_name))
            } else {
                return Err(Error::new(ErrorKind::NotFound, "Could not determine the user's home directory"));
            }
        }
    }
}
