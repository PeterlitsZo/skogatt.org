#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::State;
use rocket::response::status;
use std::sync::atomic::{AtomicU64, Ordering};

// State
// ----------------------------------------------------------------------------
struct LikeCount {
    like: AtomicU64,
    dislike: AtomicU64,
}

// Rounter
// ----------------------------------------------------------------------------
#[get("/home/like")]
fn get_like(like_count: State<LikeCount>) -> status::Accepted<String> {
    status::Accepted(Some(format!("{}", like_count.like.load(Ordering::Relaxed))))
}

#[post("/home/like")]
fn post_like(like_count: State<LikeCount>) {
    like_count.like.fetch_add(1, Ordering::Relaxed);
}

#[get("/home/dislike")]
fn get_dislike(like_count: State<LikeCount>) -> status::Accepted<String> {
    status::Accepted(Some(format!("{}", like_count.dislike.load(Ordering::Relaxed))))
}

#[post("/home/dislike")]
fn post_dislike(like_count: State<LikeCount>) {
    like_count.dislike.fetch_add(1, Ordering::Relaxed);
}


// Main
// ----------------------------------------------------------------------------
fn main() {
    rocket::ignite()
        .manage(LikeCount { like: AtomicU64::new(0), dislike: AtomicU64::new(0) })
        .mount("/api/v1", routes![get_like, post_like, get_dislike, post_dislike])
        .launch();
}
