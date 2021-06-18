use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

use log::info;

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub like: u64,
    pub dislike: u64
}

pub fn add_like(data: Arc<Mutex<Data>>) {
    let like = &mut data.lock().unwrap().like;
    *like += 1;
    info!("  like => {}", like);
}

pub fn get_like_json(data: Arc<Mutex<Data>>) -> String {
    let like = data.lock().unwrap().like;
    // return JSON String {"like": <like-number>}
    format!("{{\"like\": {}}}", like)
}

pub fn add_dislike(data: Arc<Mutex<Data>>) {
    let dislike = &mut data.lock().unwrap().dislike;
    *dislike += 1;
    info!("  dislike => {}", dislike);
}

pub fn get_dislike_json(data: Arc<Mutex<Data>>) -> String {
    let dislike = data.lock().unwrap().dislike;
    // return JSON String like {"dislike": <dislike-number>}
    format!("{{\"dislike\": {}}}", dislike)
}
