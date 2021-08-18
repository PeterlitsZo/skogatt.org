use std::collections::HashMap;
use std::str::from_utf8;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use log::info;
use serde_json::to_string;
use hyper::{Body, Method, Response, Request, StatusCode};
use hyper::body::to_bytes;

#[derive(Serialize, Deserialize, Debug)]
struct Comments {
    id: u64,
    ip: String,
    datetime: String,
    content: String
}

/// Get the comments by given argument `conn`(SQLite connection) and `page` of
/// comments
fn get_comments(conn: Arc<Mutex<Connection>>, page: i64) -> String {
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
    let length = (length + 19) / 20; // == ceil(length / 20)
    let mut result = HashMap::new();
    result.insert("result", to_string(&comments).unwrap());
    result.insert("length", to_string(&length).unwrap());

    info!("  Get comments form {} to {}", offset, offset + 20);
    to_string(&result).unwrap()
}

fn add_comment(
        conn: Arc<Mutex<Connection>>,
        ip: String,
        time: DateTime<Utc>,
        text: &str,
        response: &mut Response<Body>,
        with_time_limit: bool) {
    // Get the basic infomation
    let time = time.to_rfc3339();

    // Get the last datetime of the comments that sended by the same IP
    let mut ok = true;
    let conn = conn.lock().unwrap();
    if with_time_limit {
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

        if lasttime.len() == 1 {
            let lasttime = DateTime::parse_from_rfc3339(&lasttime[0]).unwrap();
            let lasttime = lasttime.with_timezone(&Utc);
            ok = Utc::now() - lasttime > Duration::seconds(3);
        }
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

fn get(req: Request<Body>, conn: Arc<Mutex<Connection>>) -> Response<Body> {
    // Parse URL's query to hashmap.
    let params = req.uri().query();
    let mut response = Response::new(Body::empty());
    let params: HashMap<String, String> = params
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);
    info!("  Get argument of `/api/v1/home/comments`: {:?}", params);
    let page = match params.get("page") {
        Some(number) => number.parse::<i64>().unwrap_or(1) - 1,
        None => 0
    };

    *response.body_mut() = Body::from(get_comments(conn.clone(), page));
    return response;
}

async fn post(req: Request<Body>, conn: Arc<Mutex<Connection>>) -> Response<Body> {
    let mut response = Response::new(Body::empty());

    let ip = req.headers()["x-forwarded-for"].to_str().unwrap()
        .to_string();
    let text = to_bytes(req.into_body()).await.unwrap();
    let text = from_utf8(&text).unwrap();
    add_comment(conn.clone(), ip, Utc::now(), text, &mut response, true);

    return response;
}

pub async fn handle(req: Request<Body>, conn: Arc<Mutex<Connection>>) -> Response<Body> {
    let not_found = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap();
    return match req.method() {
        &Method::GET => get(req, conn),
        &Method::POST => post(req, conn).await,
        _ => not_found,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::database;

    fn init() -> Arc<Mutex<Connection>> {
        let conn = Connection::open(database::DB_FILE_NAME).unwrap();
        let conn = Arc::new(Mutex::new(conn));
        database::init_sqlite(conn.clone());
        return conn.clone();
    }

    fn make_get_request() -> Request<Body> {
        Request::builder()
            .method(Method::GET)
            .body(Body::empty())
            .unwrap()
    }

    async fn hashmap_body(body: Body) -> HashMap<String, String> {
        let body_vec_u8 = hyper::body::to_bytes(body).await
            .unwrap().to_vec();
        let body_json = String::from_utf8(body_vec_u8).unwrap();
        return serde_json::from_str(&body_json).unwrap();
    }

    #[actix_rt::test]
    async fn test_get() {
        let _shared = database::tests::DB_FILE_RESOURCE.lock();

        database::tests::remove_sqlite_file();
        let conn = init();
        let response = handle(make_get_request(), conn.clone()).await;
        let response_body = response.into_body();

        // in the new database, the length of it is 0
        assert_eq!(hashmap_body(response_body).await.get(&"length".to_string()).unwrap(), "0");
        let mut response = Response::new(Body::empty());
        add_comment(conn.clone(), "127.0.0.1".to_string(), Utc::now(), "baka", &mut response, false); 
        // after post a comments, the length is 1 and we can get the comments.
        let response = handle(make_get_request(), conn.clone()).await;
        assert_eq!(hashmap_body(response.into_body()).await.get(&"length".to_string()).unwrap(), "1");

        // after post 43 comments, the length is 3 (because ceil(44 / 20) == 3).
        for _ in 0..43 {
            let mut response = Response::new(Body::empty());
            add_comment(conn.clone(), "127.0.0.1".to_string(), Utc::now(), "baka",
                        &mut response, false);
        }
        let response = handle(make_get_request(), conn.clone()).await;
        assert_eq!(hashmap_body(response.into_body()).await.get(&"length".to_string()).unwrap(), "3");
    }

    #[actix_rt::test]
    async fn test_post() {
        let _shared = database::tests::DB_FILE_RESOURCE.lock();

        database::tests::remove_sqlite_file();
        let conn = init();
        let response = handle(make_get_request(), conn.clone()).await;

        assert_eq!(hashmap_body(response.into_body()).await.get(&"length".to_string()).unwrap(), "0");

        // add 40 comments with time limit => only 1 comments in sqlite database.
        for i in 0..40 {
            let mut response = Response::new(Body::empty());
            add_comment(conn.clone(), "127.0.0.1".to_string(), Utc::now(), "baka",
                        &mut response, true);
            let get_response = handle(make_get_request(), conn.clone()).await;
            lazy_static! {
                static ref RE: Regex = Regex::new(
                    r#"(?x)
                    \[\{
                        "id":1,
                        "ip":"127.0.0.1",
                        "datetime":"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{9}\+\d{2}:\d{2}",
                        "content":"baka"
                    \}\]
                    "#
                ).unwrap();
            }
            assert!(RE.is_match(
                hashmap_body(get_response.into_body()).await
                    .get(&"result".to_string()).unwrap()
            ));
            if i == 0 {
                assert_eq!(response.status(), StatusCode::OK);
            } else {
                assert_eq!(response.status(), StatusCode::FORBIDDEN);
            }
        }
    }

    #[actix_rt::test]
    async fn test_unkown() {
        let _shared = database::tests::DB_FILE_RESOURCE.lock();

        let conn = init();
        let put_request = Request::builder()
            .method(Method::PUT)
            .body(Body::empty())
            .unwrap();
        let response = handle(put_request, conn.clone()).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
