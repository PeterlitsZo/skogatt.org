use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

use hyper::{Body, Response, Request, Method, StatusCode};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub like: u64,
    pub dislike: u64
}

fn save(data: &Data) {
    let json_text = serde_json::to_string(data).unwrap();
    info!("  write to file for saving");
    File::create(crate::database::JSON_FILE_NAME).unwrap()
        .write(json_text.as_bytes()).unwrap();
}

fn add_like(data: Arc<Mutex<Data>>) -> Response<Body> {
    let data = &mut data.lock().unwrap();
    data.like += 1;
    info!("  like => {}", data.like);
    save(data);
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap()
}

fn get_like_json(data: Arc<Mutex<Data>>) -> Response<Body> {
    let like = data.lock().unwrap().like;
    // return JSON String {"like": <like-number>}
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(format!("{{\"like\": {}}}", like)))
        .unwrap()
}

fn add_dislike(data: Arc<Mutex<Data>>) -> Response<Body> {
    let data = &mut data.lock().unwrap();
    data.dislike += 1;
    info!("  dislike => {}", data.dislike);
    save(data);
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap()
}

fn get_dislike_json(data: Arc<Mutex<Data>>) -> Response<Body> {
    let dislike = data.lock().unwrap().dislike;
    // return JSON String like {"dislike": <dislike-number>}
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(format!("{{\"dislike\": {}}}", dislike)))
        .unwrap()
}

pub async fn handle(req: Request<Body>, data: Arc<Mutex<Data>>) -> Response<Body> {
    let not_found = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap();
    match (req.method(), req.uri().path()) {
        (method, "/api/v1/home/like") => match method {
            &Method::GET => get_like_json(data.clone()),
            &Method::POST => add_like(data.clone()),
            _ => not_found,
        },
        (method, "/api/v1/home/dislike") => match method {
            &Method::GET => get_dislike_json(data.clone()),
            &Method::POST => add_dislike(data.clone()),
            _ => not_found,
        },
        _ => not_found,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database;

    use std::collections::HashMap;

    fn make_get_request(uri: &str) -> Request<Body> {
        Request::builder()
            .method(Method::GET)
            .uri(uri)
            .body(Body::empty())
            .unwrap()
    }

    fn make_post_request(uri: &str) -> Request<Body> {
        Request::builder()
            .method(Method::POST)
            .uri(uri)
            .body(Body::empty())
            .unwrap()
    }

    async fn hashmap_body(body: Body) -> HashMap<String, i32> {
        let body_vec_u8 = hyper::body::to_bytes(body).await
            .unwrap().to_vec();
        let body_json = String::from_utf8(body_vec_u8).unwrap();
        return serde_json::from_str(&body_json).unwrap();
    }

    #[actix_rt::test]
    async fn test_get_and_post() {
        let _shared = database::tests::JSON_FILE_RESOURCE.lock();

        database::tests::remove_data_json_file();
        let mut data = Data{like: 0, dislike: 0};
        database::init_data_json_file(&mut data, database::JSON_FILE_NAME).unwrap();
        let data = Arc::new(Mutex::new(data));

        // in the new json, the counter `like` = 1, and `dislike` = 1 as well.
        let response = handle(make_get_request("/api/v1/home/like"), data.clone()).await;
        assert_eq!(*hashmap_body(response.into_body()).await.get(&"like".to_string()).unwrap(), 0 as i32);

        let response = handle(make_get_request("/api/v1/home/dislike"), data.clone()).await;
        assert_eq!(*hashmap_body(response.into_body()).await.get(&"dislike".to_string()).unwrap(), 0 as i32);

        // after post 43 times, the number of counter `like` = 43
        for _ in 0..43 {
            handle(make_post_request("/api/v1/home/like"), data.clone()).await;
        }
        let response = handle(make_get_request("/api/v1/home/like"), data.clone()).await;
        assert_eq!(*hashmap_body(response.into_body()).await.get(&"like".to_string()).unwrap(), 43 as i32);

        let response = handle(make_get_request("/api/v1/home/dislike"), data.clone()).await;
        assert_eq!(*hashmap_body(response.into_body()).await.get(&"dislike".to_string()).unwrap(), 0 as i32);

        // even reinit, the data never change.
        let mut data = Data{like: 0, dislike: 0};
        database::init_data_json_file(&mut data, database::JSON_FILE_NAME)
            .unwrap();
        let data = Arc::new(Mutex::new(data));
        let response = handle(make_get_request("/api/v1/home/like"), data.clone()).await;
        assert_eq!(*hashmap_body(response.into_body()).await.get(&"like".to_string()).unwrap(), 43 as i32);
    }
}
