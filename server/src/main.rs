// extern crate env_logger;
// extern crate hyper;
// extern crate tokio;
// #[macro_use] extern crate log;

use std::{convert::Infallible, net::SocketAddr};
use std::sync::{Arc, Mutex};

use hyper::{Body, Response, Server, Request, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use log::{info};

fn handle(req: Request<Body>, like: Arc<Mutex<u64>>, dislike: Arc<Mutex<u64>>)
        -> Response<Body>
{
    let mut response = Response::new(Body::empty());

    info!("{} {}", req.method(), req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api/v1/home/like") => {
            *response.body_mut() = Body::from(format!("{}", *like.lock().unwrap()));
        },
        (&Method::POST, "/api/v1/home/like") => {
            let mut like = like.lock().unwrap();
            *like += 1;
            info!("like => {}", *like);
        },
        (&Method::GET, "/api/v1/home/dislike") => {
            *response.body_mut() = Body::from(format!("{}", *dislike.lock().unwrap()));
        },
        (&Method::POST, "/api/v1/home/dislike") => {
            let mut dislike = like.lock().unwrap();
            *dislike += 1;
            info!("dislike => {}", *dislike);
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    response
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8100));

    let like = Arc::new(Mutex::new(0));
    let dislike = Arc::new(Mutex::new(0));

    let make_svc = make_service_fn(move |_conn| {
        let like = like.clone();
        let dislike = dislike.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let like = like.clone();
                let dislike = dislike.clone();
                async move {
                    Ok::<_, Infallible>(handle(req, like, dislike))
                }
            }))
        }
    });

    info!("Listen to {}", addr);
    Server::bind(&addr).serve(make_svc).await?;
    Ok(())
}
