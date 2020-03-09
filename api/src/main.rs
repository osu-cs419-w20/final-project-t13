#[macro_use] extern crate serde;

use std::env;

use warp::Filter;

mod db;
mod error;
mod filters;
mod handlers;

use error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = db::DB::new()?;

    let api = filters::build(db);
    let music_root = env::var("MUSIC_DIR_ROOT").expect("MUSIC_DIR_ROOT environment variable not set");
    let music_files = warp::path("static")
        .and(warp::fs::dir(music_root));

    let routes = api.or(music_files);

    let port = env::var("API_PORT").map(|p| u16::from_str_radix(&p, 10).expect("Failed to parse port")).expect("API_PORT environment variable not set");
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;

    Ok(())
}
