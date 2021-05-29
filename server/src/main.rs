use std::{env};
use std::convert::{Infallible};
use std::fs::{File};
use std::io::{Read, Write};
use std::net::{SocketAddr};
use std::str::{from_utf8};
use std::sync::{Arc, Mutex};

use chrono::{Utc};
use hyper::{Body, Response, Server, Request, Method, StatusCode};
use hyper::body::{to_bytes};
use hyper::server::conn::{AddrStream};
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
          conn: Arc<Mutex<Connection>>,
          addr: SocketAddr)
        -> Response<Body>
{
    let mut response = Response::new(Body::empty());
    let mut write = false;

    info!("{} {}", req.method(), req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api/v1/home/like") => {
            let like = data.lock().unwrap().like;
            // *response.headers_mut() = ContentType::json();
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
            let mut stmt = conn.prepare("
                SELECT id, ip, datetime, content FROM comments;
            ").unwrap();
            let result = stmt.query_map([], |row| {
                Ok(Comments{
                    id: row.get(0).unwrap(),
                    ip: row.get(1).unwrap(),
                    datetime: row.get(2).unwrap(),
                    content: row.get(3).unwrap()
                })
            });
            let result: Vec<Comments> = result.unwrap()
                .map(|c| c.unwrap())
                .collect();
            info!("Comments: {:?}", result);
            let json_text = serde_json::to_string(&result).unwrap();
            *response.body_mut() = Body::from(json_text);
        },
        (&Method::POST, "/api/v1/home/comments") => {
            let ip = format!("{}", addr.ip());
            let time = Utc::now().to_rfc3339();
            let text = to_bytes(req.into_body()).await.unwrap();
            let text = from_utf8(&text).unwrap();
            info!("Insert to table comments: ($id, {}, {}, {})", ip, text, time);
            conn.lock().unwrap().execute("
                INSERT INTO comments (ip, datetime, content)
                    VALUES(?1, ?2, ?3);
            ", params![ip, time, text]).unwrap();
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

    info!("Response: {:?}", response);
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
    let addr = SocketAddr::from(([127, 0, 0, 1], 8100));
    let make_svc = make_service_fn(move |web_conn: &AddrStream| {
        let data = data.clone();
        let conn = conn.clone();
        let addr = web_conn.remote_addr();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let data = data.clone();
                let conn = conn.clone();
                async move {
                    Ok::<_, Infallible>(handle(req, data, conn, addr).await)
                }
            }))
        }
    });

    // run the server
    info!("Listen to {}", addr);
    Server::bind(&addr).serve(make_svc).await?;
    Ok(())
}
