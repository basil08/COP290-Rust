mod graph;
mod parser;

use std::env;
use axum::{
    routing::{get, post},
    extract::Json as ExtractJson,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Cell {
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Sheet {
    data: Vec<Vec<Cell>>,
}

// Define request payload for updating a cell
#[derive(Serialize, Deserialize, Debug)]
struct UpdateCellRequest {
    row_id: String,
    column_id: String,
    value: String,
    formula: String, // this will be parsed
}

// Define response for update operations
#[derive(Serialize, Deserialize, Debug)]
struct UpdateResponse {
    success: bool,
    message: String,
}

// Create a shared state for the sheet data
type AppState = Arc<RwLock<Vec<i32>>>;

async fn get_sheet(state: axum::extract::State<AppState>) -> Json<Sheet> {
    let sheet = state.read().await.clone();
    Json(sheet)
}

async fn update_cell(
    axum::extract::State(state): axum::extract::State<AppState>,
    ExtractJson(payload): ExtractJson<UpdateCellRequest>,
) -> Json<UpdateResponse> {
    // Parse row and column IDs from string to usize
    let row_index = match payload.row_id.parse::<usize>() {
        Ok(index) => index,
        Err(_) => {
            return Json(UpdateResponse {
                success: false,
                message: "Invalid row ID format".to_string(),
            })
        }
    };

    let col_index = match payload.column_id.parse::<usize>() {
        Ok(index) => index,
        Err(_) => {
            return Json(UpdateResponse {
                success: false,
                message: "Invalid column ID format".to_string(),
            })
        }
    };

    // Update the cell value in our sheet data
    let mut graph = state.write().await;

    let mut trimmed = payload.formula.trim();

    let status = parser(trimmed, c, r, &mut arr, &mut graph, &mut formula_array[..]);



    
    // Check if the indices are valid
    if row_index >= sheet.data.len() || col_index >= sheet.data[0].len() {
        return Json(UpdateResponse {
            success: false,
            message: "Cell indices out of bounds".to_string(),
        });
    }

    // Update the cell value
    sheet.data[row_index][col_index].value = payload.value;

    // Return success response
    Json(UpdateResponse {
        success: true,
        message: "Cell updated successfully".to_string(),
    })
}

#[tokio::main]
async fn main() {
    // Initialize the sheet with default values
    // let initial_sheet = Sheet {
    //     data: vec![vec![Cell { value: "0".into() }; 10]; 10],
    // };

    use graph::{Graph, Formula};
    use parser::parser;
    
    let mut graph = Graph::new(100);
    let mut formula_array = vec![Formula::default(); 100];
    let mut arr = vec![0; 100];

    let mut currx = 0;
    let mut curry = 0;
    
    // Create the shared state
    let app_state = Arc::new(RwLock::new(graph));

    // Create a CORS layer that allows any origin
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Add the CORS layer to your router
    let app = Router::new()
        .route("/sheet", get(get_sheet))
        .route("/update-cell", post(update_cell))
        .with_state(app_state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("✅ Server running at http://{}", addr);
    println!("🔄 Cell update endpoint available at http://{}/update-cell", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}