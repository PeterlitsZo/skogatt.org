mod home;
mod database;

use std::collections::HashMap;
use std::convert::Infallible;
use std::fs::File;
use std::io::Write;
use std::net::SocketAddr;
use std::str::from_utf8;
use std::error;
use std::sync::{Arc, Mutex};

use chrono::Utc;
use hyper::{Body, Response, Server, Request, Method, StatusCode};
use hyper::body::to_bytes;
use hyper::service::{make_service_fn, service_fn};
use log::info;
use rusqlite::{Connection, Result};

use home::{like, comments};
use database::{JSON_FILE_NAME, DB_FILE_NAME, init_data_json_file, init_sqlite};

/// Core function:
///   - Get the request and the pointer to mut data
///   - Deal with those
///   - Return a response
async fn handle(req: Request<Body>,
                data: Arc<Mutex<like::Data>>,
                conn: Arc<Mutex<Connection>>)
        -> Response<Body>
{
    let mut response = Response::new(Body::empty());
    let mut write = false;

    info!("{} {}", req.method(), req.uri());
    match (req.method(), req.uri().path()) {
        (method, "/api/v1/home/like") => match method {
            &Method::GET => {
                *response.body_mut() = Body::from(like::get_like_json(data.clone()));
            },
            &Method::POST => {
                like::add_like(data.clone());
                write = true;
            },
            _ => {
                *response.status_mut() = StatusCode::NOT_FOUND;
            },
        },
        (method, "/api/v1/home/dislike") => match method {
            &Method::GET => {
                *response.body_mut() = Body::from(like::get_dislike_json(data.clone()));
            },
            &Method::POST => {
                like::add_dislike(data.clone());
                write = true;
            },
            _ => {
                *response.status_mut() = StatusCode::NOT_FOUND;
            },
        }
        (method, "/api/v1/home/comments") => match method {
            &Method::GET => {
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
                let page = match params.get("page") {
                    Some(number) => number.parse::<i64>().unwrap_or(1) - 1,
                    None => 0
                };

                *response.body_mut() = Body::from(comments::get_comments(conn.clone(), page));
            },
            &Method::POST => {
                let ip = req.headers()["x-forwarded-for"].to_str().unwrap()
                    .to_string();
                let text = to_bytes(req.into_body()).await.unwrap();
                let text = from_utf8(&text).unwrap();
                comments::add_comment(conn.clone(), ip, Utc::now(), text, &mut response);
            },
            _ => {
                *response.status_mut() = StatusCode::NOT_FOUND;
            },
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

pub async fn init()
        -> Result<(Arc<Mutex<like::Data>>, Arc<Mutex<Connection>>),
                  Box<dyn error::Error>> {
    // init logger and data struct
    env_logger::init();

    info!("Going to read json file {} to init", JSON_FILE_NAME);
    let mut data = like::Data{like: 0, dislike: 0};
    init_data_json_file(&mut data, JSON_FILE_NAME)?;
    let data = Arc::new(Mutex::new(data));

    info!("Going to connect SQLite file {} and init", DB_FILE_NAME);
    let conn = Connection::open(DB_FILE_NAME)?;
    let conn = Arc::new(Mutex::new(conn));
    init_sqlite(conn.clone());

    Ok((data, conn))
}

pub async fn run(data: Arc<Mutex<like::Data>>, conn: Arc<Mutex<Connection>>)
        -> Result<(), Box<dyn std::error::Error>> {
    // run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8100));
    info!("Listen to {}", addr);
    Server::bind(&addr).serve(
        make_service_fn(move |_conn| {
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
        })).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(2 + 2 == 4);
    }
}
