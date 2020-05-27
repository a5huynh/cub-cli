extern crate dirs;
use std::path::Path;

static APP_PATHS: [&str; 2] = [
    "Library/Containers/net.shinyfrog.bear/Data/Documents",
    "Library/Group Containers/9K33E3U3T4.net.shinyfrog.bear",
];

const DB_PATH: &str = "Application Data/database.sqlite";

pub fn find_db() -> Result<String, &'static str> {
    let home = dirs::home_dir().unwrap();

    for path in APP_PATHS.iter() {
        let path = home.join(format!("{}/{}", path, DB_PATH));
        if let Some(path_str) = path.to_str() {
            if Path::new(&path_str).exists() {
                return Ok(String::from(path_str));
            }
        }
    }

    Err("Unable to find Bear database.")
}
