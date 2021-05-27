use std::convert::{Infallible};
use std::net::{SocketAddr};
use std::sync::{Arc, Mutex};
use std::fs::{File};
use std::io::{Read, Write};

use hyper::{Body, Response, Server, Request, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use log::{info};
use serde::{Deserialize, Serialize};

static JSON_FILE_NAME: &str = "data.json";

#[derive(Serialize, Deserialize)]
struct Data {
    like: u64,
    dislike: u64
}

/// Core function:
///   - Get the request and the pointer to mut data
///   - Deal with those
///   - Return a response
fn handle(req: Request<Body>, data: Arc<Mutex<Data>>)
        -> Response<Body>
{
    let mut response = Response::new(Body::empty());
    let mut write = false;

    info!("{} {}", req.method(), req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api/v1/home/like") => {
            let like = data.lock().unwrap().like;
            *response.body_mut() = Body::from(format!("{}", like));
        },
        (&Method::POST, "/api/v1/home/like") => {
            let like = &mut data.lock().unwrap().like;
            *like += 1;
            write = true;
            info!("  like => {}", like);
        },
        (&Method::GET, "/api/v1/home/dislike") => {
            let dislike = data.lock().unwrap().dislike;
            *response.body_mut() = Body::from(format!("{}", dislike));
        },
        (&Method::POST, "/api/v1/home/dislike") => {
            let dislike = &mut data.lock().unwrap().dislike;
            *dislike += 1;
            write = true;
            info!("  dislike => {}", dislike);
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

    response
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Read file {} to init", JSON_FILE_NAME);
    let mut data = Data{like: 0, dislike: 0};
    match File::open(JSON_FILE_NAME) {
        Ok(mut file) => {
            let mut json_data = "".to_string();
            file.read_to_string(&mut json_data).unwrap();
            data = serde_json::from_str(&json_data).unwrap();
        },
        Err(_) => {
            let mut file = File::create(JSON_FILE_NAME).unwrap();
            let content: &str = &serde_json::to_string(&data).unwrap();
            file.write(content.as_bytes()).unwrap();
        }
    };
    let data = Arc::new(Mutex::new(data));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8100));

    let make_svc = make_service_fn(move |_conn| {
        let data = data.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let data = data.clone();
                async move {
                    Ok::<_, Infallible>(handle(req, data))
                }
            }))
        }
    });

    info!("Listen to {}", addr);
    Server::bind(&addr).serve(make_svc).await?;
    Ok(())
}
