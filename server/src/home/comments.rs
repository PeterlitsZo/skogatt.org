use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use log::info;
use serde_json::to_string;
use hyper::{Body, Response, StatusCode};

#[derive(Serialize, Deserialize, Debug)]
struct Comments {
    id: u64,
    ip: String,
    datetime: String,
    content: String
}

/// Get the comments by given argument `conn`(SQLite connection) and `page` of
/// comments
pub fn get_comments(conn: Arc<Mutex<Connection>>, page: i64) -> String {
    let conn = conn.lock().unwrap();

    // Get the table's lenth of table `comments` and calc the result's
    // offset.
    let length: Vec<i64> = conn.prepare("
        SELECT COUNT(*) FROM comments;
    ").unwrap()
        .query_map([], |row| row.get(0)).unwrap()
        .map(|c| c.unwrap())
        .collect();
    assert!(length.len() == 1);
    let length = length[0];
    let offset = 20 * page;

    // Get the comments data from database.
    let mut stmt = conn.prepare("
        SELECT id, ip, datetime, content
            FROM comments
            ORDER BY id DESC
            LIMIT 20 OFFSET ?;
    ").unwrap();

    let comments: Vec<Comments> = stmt
        .query_map([&offset], |row| {
            Ok(Comments{
                id: row.get(0).unwrap(),
                ip: row.get(1).unwrap(),
                datetime: row.get(2).unwrap(),
                content: row.get(3).unwrap()
            })
        })
        .unwrap()
        .map(|c| c.unwrap())
        .collect();

    // Build the JSON result.
    let length = (length + 19) / 20;
    let mut result = HashMap::new();
    result.insert("result", to_string(&comments).unwrap());
    result.insert("length", to_string(&length).unwrap());

    info!("  Get comments form {} to {}", offset, offset + 20);
    to_string(&result).unwrap()
}

pub fn add_comment(
        conn: Arc<Mutex<Connection>>,
        ip: String,
        time: DateTime<Utc>,
        text: &str,
        response: &mut Response<Body>) {
    // Get the basic infomation
    let time = time.to_rfc3339();

    // Get the last datetime of the comments that sended by the same IP
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare("
        SELECT datetime FROM comments
            WHERE ip = ?
            ORDER BY datetime(datetime) DESC
            LIMIT 1;
    ").unwrap();
    let lasttime: Vec<String> = stmt
        .query_map([&ip], |row| row.get(0)).unwrap()
        .map(|c| c.unwrap())
        .collect();
    assert!(lasttime.len() <= 1);

    let mut ok = true;
    if lasttime.len() == 1 {
        let lasttime = DateTime::parse_from_rfc3339(&lasttime[0]).unwrap();
        let lasttime = lasttime.with_timezone(&Utc);
        ok = Utc::now() - lasttime > Duration::seconds(3);
    }

    // Raise error(403) if the last time the client send is too close(3s)
    if !ok {
        *response.status_mut() = StatusCode::FORBIDDEN;
    // Or OK(200)
    } else {
        info!("  Insert to table comments: ($id, {}, {:.12}..., {})", ip, text, time);
        conn.execute("
            INSERT INTO comments (ip, datetime, content)
                VALUES(?1, ?2, ?3);
        ", params![ip, time, text]).unwrap();
    }
}
