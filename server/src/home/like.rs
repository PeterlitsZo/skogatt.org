use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

use log::{info};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub like: u64,
    pub dislike: u64
}

pub fn addLike(data: Arc<Mutex<Data>>) {
    let like = &mut data.lock().unwrap().like;
    *like += 1;
    info!("  like => {}", like);
}

pub fn getLikeJSON(data: Arc<Mutex<Data>>) -> String {
    let like = data.lock().unwrap().like;
    // return JSON String {"like": <like-number>}
    format!("{{\"like\": {}}}", like)
}

pub fn addDislike(data: Arc<Mutex<Data>>) {
    let dislike = &mut data.lock().unwrap().dislike;
    *dislike += 1;
    info!("  dislike => {}", dislike);
}

pub fn getDislikeJSON(data: Arc<Mutex<Data>>) -> String {
    let dislike = data.lock().unwrap().dislike;
    // return JSON String like {"dislike": <dislike-number>}
    format!("{{\"dislike\": {}}}", dislike)
}
