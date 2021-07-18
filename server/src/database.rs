use std::error;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{Read, Write};

use rusqlite::Connection;
use log::info;

use crate::home::like::Data;

pub static JSON_FILE_NAME: &'static str = env!("JSON_FILE_NAME");
pub static DB_FILE_NAME: &'static str = env!("DB_FILE_NAME");

/// init data struct by path
pub fn init_data_json_file(data: &mut Data, path: &str)
        -> Result<(), Box<dyn error::Error>> {
    match File::open(path) {
        // find file => read and init `data`.
        Ok(mut file) => {
            info!("Find {}. Read it.", path);
            let mut json_data = "".to_string();
            file.read_to_string(&mut json_data)?;
            *data = serde_json::from_str(&json_data)?;
        },
        // cannot find file => create and write
        Err(_) => {
            info!("Cannot Find {}. Create and write it.", path);
            let mut file = File::create(JSON_FILE_NAME)?;
            let content: &str = &serde_json::to_string(&data)?;
            file.write(content.as_bytes())?;
        }
    };
    Ok(())
}

/// init SQLite by path
pub fn init_sqlite(conn: Arc<Mutex<Connection>>) {
    let conn = conn.lock().unwrap();
    conn.execute("
        CREATE TABLE IF NOT EXISTS comments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ip TEXT,
            datetime TEXT,
            content TEXT
        );
    ", []).unwrap();
}
