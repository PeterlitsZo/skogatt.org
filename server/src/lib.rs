mod home;
mod database;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::error;
use std::sync::{Arc, Mutex};

use hyper::{Body, Response, Server, Request, StatusCode};
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
    let not_found = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap();

    info!("{} {}", req.method(), req.uri());
    match req.uri().path() {
        "/api/v1/home/like" => like::handle(req, data.clone()).await,
        "/api/v1/home/dislike" => like::handle(req, data.clone()).await,
        "/api/v1/home/comments" => comments::handle(req, conn).await,
        _ => not_found,
    }
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
