use std::{env};
use std::array::{IntoIter};
use std::collections::{HashMap};
use std::convert::{Infallible};
use std::fs::{File};
use std::io::{Read, Write};
use std::iter::{FromIterator};
use std::net::{SocketAddr};
use std::str::{from_utf8};
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Utc};
use hyper::{Body, Response, Server, Request, Method, StatusCode};
use hyper::body::{to_bytes};
use hyper::service::{make_service_fn, service_fn};
use log::{info};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

static JSON_FILE_NAME: &'static str = env!("JSON_FILE_NAME");
static DB_FILE_NAME: &'static str = env!("DB_FILE_NAME");

#[derive(Serialize, Deserialize)]
struct Data {
    like: u64,
    dislike: u64
}

#[derive(Serialize, Deserialize, Debug)]
struct Comments {
    id: u64,
    ip: String,
    datetime: String,
    content: String
}

/// Core function:
///   - Get the request and the pointer to mut data
///   - Deal with those
///   - Return a response
async fn handle(req: Request<Body>,
                data: Arc<Mutex<Data>>,
                conn: Arc<Mutex<Connection>>)
        -> Response<Body>
{
    let mut response = Response::new(Body::empty());
    let mut write = false;

    info!("{} {}", req.method(), req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api/v1/home/like") => {
            let like = data.lock().unwrap().like;
            *response.body_mut() = Body::from(format!("{{\"like\":{}}}", like));
        },
        (&Method::POST, "/api/v1/home/like") => {
            let like = &mut data.lock().unwrap().like;
            *like += 1;
            write = true;
            info!("  like => {}", like);
        },
        (&Method::GET, "/api/v1/home/dislike") => {
            let dislike = data.lock().unwrap().dislike;
            // *response.headers_mut() = ContentType::json();
            *response.body_mut() = Body::from(format!("{{\"dislike\":{}}}", dislike));
        },
        (&Method::POST, "/api/v1/home/dislike") => {
            let dislike = &mut data.lock().unwrap().dislike;
            *dislike += 1;
            write = true;
            info!("  dislike => {}", dislike);
        },
        (&Method::GET, "/api/v1/home/comments") => {
            let conn = conn.lock().unwrap();

            // Parse URL's query to hashmap.
            let params = req.uri().query();
            let params: HashMap<String, String> = params
                .map(|v| {
                    url::form_urlencoded::parse(v.as_bytes())
                        .into_owned()
                        .collect()
                })
                .unwrap_or_else(HashMap::new);
            info!("  Get argument of `/api/v1/home/comments`: {:?}", params);

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
            let offset = 20 * match params.get("page") {
                Some(number) => number.parse::<i64>().unwrap_or(1) - 1,
                None => 0
            };

            // Get the data from database.
            let mut stmt = conn.prepare("
                SELECT id, ip, datetime, content
                    FROM comments
                    ORDER BY id DESC
                    LIMIT 20 OFFSET ?;
            ").unwrap();

            let result = stmt.query_map([&offset], |row| {
                Ok(Comments{
                    id: row.get(0).unwrap(),
                    ip: row.get(1).unwrap(),
                    datetime: row.get(2).unwrap(),
                    content: row.get(3).unwrap()
                })
            }).unwrap();
            let result: Vec<Comments> = result
                .map(|c| c.unwrap())
                .collect();
            let length = (length + 19) / 20;
            let result = HashMap::<_, _>::from_iter(IntoIter::new([
                    ("result", serde_json::to_string(&result).unwrap()),
                    ("length", serde_json::to_string(&length).unwrap()),
                ]));

            // Convert to json to response.
            info!("  Get comments form {} to {}", offset, offset + 20);
            let json_text = serde_json::to_string(&result).unwrap();
            *response.body_mut() = Body::from(json_text);
        },
        (&Method::POST, "/api/v1/home/comments") => {
            // Get the basic infomation
            let ip = {
                let ip = req.headers()["x-forwarded-for"].to_str().unwrap();
                ip.to_string()
            };
            let time = Utc::now().to_rfc3339();
            let text = to_bytes(req.into_body()).await.unwrap();
            let text = from_utf8(&text).unwrap();

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

            // Raise error(403) if the last time the client send is too close
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
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };
   
    // write => write data to the json file to save
    if write {
        let json_text = serde_json::to_string(&*data.lock().unwrap()).unwrap();
        info!("  write to file for saving");
        File::create(JSON_FILE_NAME).unwrap()
            .write(json_text.as_bytes()).unwrap();
    }

    info!("  Return response");
    response
}

/// init data struct by path
fn init_data_json_file(data: &mut Data, path: &str) {
    match File::open(path) {
        // find file => read and init `data`.
        Ok(mut file) => {
            info!("Find {}. Read it.", path);
            let mut json_data = "".to_string();
            file.read_to_string(&mut json_data).unwrap();
            *data = serde_json::from_str(&json_data).unwrap();
        },
        // cannot find file => create and write
        Err(_) => {
            info!("Cannot Find {}. Create and write it.", path);
            let mut file = File::create(JSON_FILE_NAME).unwrap();
            let content: &str = &serde_json::to_string(&data).unwrap();
            file.write(content.as_bytes()).unwrap();
        }
    };
}

/// init SQLite by path
fn init_sqlite(conn: Arc<Mutex<Connection>>) {
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

/// Main function. The entrypoint of the server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init logger and data struct
    env_logger::init();

    info!("Going to read json file {} to init", JSON_FILE_NAME);
    let mut data = Data{like: 0, dislike: 0};
    init_data_json_file(&mut data, JSON_FILE_NAME);
    let data = Arc::new(Mutex::new(data));

    info!("Going to connect SQLite file {} and init", DB_FILE_NAME);
    let conn = Connection::open(DB_FILE_NAME).unwrap();
    let conn = Arc::new(Mutex::new(conn));
    init_sqlite(conn.clone());

    // init the function to run the server
    let make_svc = make_service_fn(move |_conn| {
        let data = data.clone();
        let conn = conn.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let data = data.clone();
                let conn = conn.clone();
                async move {
                    Ok::<_, Infallible>(handle(req, data, conn).await)
                }
            }))
        }
    });

    // run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8100));
    info!("Listen to {}", addr);
    Server::bind(&addr).serve(make_svc).await?;
    Ok(())
}
