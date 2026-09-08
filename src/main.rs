use axum::{Json, Router, routing::get};
use serde::Serialize;
use std::{ error::Error, println, sync::{Arc, RwLock}, time::Duration};

#[derive(Serialize)]
struct Bookmark{
    id:u64,
    title:String,
    url:String,
}

#[derive(Clone)]
struct AppState{
    bookmarks: Arc<RwLock<Vec<Bookmark>>>
}


#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
    let state=AppState{
        bookmarks: Arc::new(RwLock::new(vec![Bookmark{
            id:1,
            title:"Rust".to_string(),
            url:"https://www.rust-lang.org/".to_string(),
        },
        Bookmark{
            id:2,
            title: "Axum".to_string(),
            url: "https://docs.rs/axum/latest/axum/".to_string(),
        }]))
    };


    let app=Router::new()

    .route("/bookmarks", get(get_bookmarks))
    .with_state(state);


    let listener=tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("http://localhost:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn hello() -> &'static str{
    tokio::time::sleep(Duration::from_secs(3)).await;

    "Hello, Rust!"
}

async fn get_bookmarks() -> Json<Vec<Bookmark>>{
    let bookmarks=vec![
        
    ];

    Json(bookmarks)
}