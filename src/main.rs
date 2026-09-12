use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;
use tokio::sync::RwLock;
use std::{ error::Error, println, sync::Arc};

#[derive(Clone,Serialize)]
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

async fn get_bookmarks(
    State(state): State<AppState>
) -> Json<Vec<Bookmark>>{
    let bookmarks=state.bookmarks.read().await;
    Json(bookmarks.clone())
}