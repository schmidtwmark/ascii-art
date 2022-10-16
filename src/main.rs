use axum::{
    http::StatusCode,
    routing::get,
    Json, Router, Extension,
};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::error::Error;
use rand::seq::SliceRandom;
use std::sync::{Arc, RwLock};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    // init tracing
    tracing_subscriber::fmt::init();
    let ascii_arts = Arc::new(RwLock::new(load_art()));


    let app = Router::new()
        .route("/random", get(get_random_art))
        .layer(Extension(ascii_arts));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();    println!("Hello, world!");

    Ok(())
}

fn load_art() -> Vec<AsciiArt> {
    fs::read_dir("art/").unwrap().enumerate().map(|(idx, path)| {
        let path = path.unwrap().path();
        let art = fs::read_to_string(&path).unwrap();
        AsciiArt { art, title: path.file_name().unwrap().to_str().unwrap().to_owned(), id: idx }
    }).collect()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AsciiArt {
    art: String,
    title: String,
    id: usize
}

async fn get_random_art(Extension(ascii_arts): Extension<Arc<RwLock<Vec<AsciiArt>>>>) -> Result<Json<AsciiArt>, StatusCode> {
    let arts = ascii_arts.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let art = arts.choose(&mut rand::thread_rng()).ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(art.clone()))
}