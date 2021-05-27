use std::{convert::Infallible, net::SocketAddr};
use std::sync::{Arc, Mutex};

use hyper::{Body, Response, Server, Request, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use log::{info};

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

    info!("{} {}", req.method(), req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api/v1/home/like") => {
            let like = data.lock().unwrap().like;
            *response.body_mut() = Body::from(format!("{}", like));
        },
        (&Method::POST, "/api/v1/home/like") => {
            let like = &mut data.lock().unwrap().like;
            *like += 1;
            info!("  like => {}", like);
        },
        (&Method::GET, "/api/v1/home/dislike") => {
            let dislike = data.lock().unwrap().dislike;
            *response.body_mut() = Body::from(format!("{}", dislike));
        },
        (&Method::POST, "/api/v1/home/dislike") => {
            let dislike = &mut data.lock().unwrap().dislike;
            *dislike += 1;
            info!("  dislike => {}", dislike);
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

    let data = Arc::new(Mutex::new(Data{like:0, dislike:0}));

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
