use std::error;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{Read, Write};

use rusqlite::Connection;
use log::info;

use crate::home::like::Data;

#[cfg(test)]
pub static JSON_FILE_NAME: &'static str = "/tmp/data.json";
#[cfg(not(test))]
pub static JSON_FILE_NAME: &'static str = "/var/server/data.json";
#[cfg(test)]
pub static DB_FILE_NAME: &'static str = "/tmp/db.sqlite";
#[cfg(not(test))]
pub static DB_FILE_NAME: &'static str = "/var/server/db.sqlite";

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

#[cfg(test)]
pub mod tests {
    use super::*;

    use std::path;
    use std::fs;
    use once_cell::sync::Lazy;

    pub static DB_FILE_RESOURCE: Lazy<Mutex<()>> = Lazy::new(Mutex::default);

    pub fn remove_data_json_file() {
        if path::Path::new(JSON_FILE_NAME).exists() == true {
            fs::remove_file(JSON_FILE_NAME).unwrap_or_else(|why| {
                println!("! {:?}", why);
            });
        }
    }

    pub fn remove_sqlite_file() {
        if path::Path::new(DB_FILE_NAME).exists() == true {
            fs::remove_file(DB_FILE_NAME).unwrap_or_else(|why| {
                println!("! {:?}", why);
            });
        }
    }

    #[test]
    fn test_init_data_json_file() {
        // remove and then init(create) a json file.
        remove_data_json_file();
        assert_eq!(path::Path::new(JSON_FILE_NAME).exists(), false);

        init_data_json_file(&mut Data{like: 10, dislike: 20}, JSON_FILE_NAME)
            .unwrap();
        assert_eq!(path::Path::new(JSON_FILE_NAME).exists(), true);

        // read its content, and assert it has the right content.
        let mut json_file = fs::File::open(JSON_FILE_NAME)
            .unwrap();
        let mut content = String::new();
        json_file.read_to_string(&mut content).unwrap();
        assert_eq!(content, r#"{"like":10,"dislike":20}"#);

        // if it is already inited, its content will not change.
        init_data_json_file(&mut Data{like: 20, dislike: 10}, JSON_FILE_NAME)
            .unwrap();
        json_file.read_to_string(&mut content).unwrap();
        assert_eq!(content, r#"{"like":10,"dislike":20}"#);
    }

    #[test]
    fn test_init_sqlite() {
        let _shared = DB_FILE_RESOURCE.lock();

        // remove and then init(create) a db file.
        remove_sqlite_file();
        assert_eq!(path::Path::new(DB_FILE_NAME).exists(), false);

        let conn = Connection::open(DB_FILE_NAME).unwrap();
        let conn = Arc::new(Mutex::new(conn));
        init_sqlite(conn.clone());
        assert_eq!(path::Path::new(DB_FILE_NAME).exists(), true);

        // read its content, its `comments` table is empty.
        let length: Vec<i64> = conn.lock().unwrap().prepare("
            SELECT COUNT(*) FROM comments;
        ").unwrap()
            .query_map([], |row| row.get(0)).unwrap()
            .map(|c| c.unwrap())
            .collect();
        assert_eq!(length.len(), 1);
        let length = length[0];
        assert_eq!(length, 0);
    }
}
