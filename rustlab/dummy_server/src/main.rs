use axum::{
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Cell {
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Sheet {
    data: Vec<Vec<Cell>>,
}

async fn get_sheet() -> Json<Sheet> {
    let sheet = Sheet {
        data: vec![vec![Cell { value: "0".into() }; 10]; 10],
    };
    Json(sheet)
}

#[tokio::main]
async fn main() {
    // Create a CORS layer that allows any origin
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Add the CORS layer to your router
    let app = Router::new()
        .route("/sheet", get(get_sheet))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("✅ Server running at http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}