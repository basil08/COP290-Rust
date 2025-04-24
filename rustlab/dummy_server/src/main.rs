mod handlers;
mod operations;
mod server_models;
mod types;

use axum::{
    routing::{get, post},
    Router,
};
use sheet::display_ext::{printer, scroller};
use sheet::function_ext::Cell;
use sheet::graph_ext::StateSnapshot;
use sheet::graph_ext::{Formula, Graph, State};
use sheet::parser_ext::parser;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

use handlers::{get_sheet, process_query, redo_action, undo_action, update_cell};
use server_models::Sheet;
use types::{AppState, ExtendedState};

// Helper function to create a snapshot (similar to the CLI version)
fn create_snapshot(arr: &Vec<Cell>, formula_array: &Vec<Formula>, graph: &Graph) -> StateSnapshot {
    StateSnapshot {
        arr: arr.clone(),
        formula_array: formula_array.clone(),
        graph: graph.clone(),
    }
}

#[tokio::main]
async fn main() {
    // Initialize the sheet with default values
    let r = 10;
    let c = 10;
    let num_cells = r * c;

    // Initialize extended state components
    let cells = vec![Cell::new_int(0); num_cells];
    let formula_array = vec![Formula::default(); num_cells];
    let graph = Graph::new(num_cells);
    let state = State::new();

    // Initialize regular sheet model for API compatibility
    let sheet = Sheet::new(10, 10);
    let num_cells = r * c;
    let cols_i32 = c as i32;
    let rows_i32 = r as i32;

    let mut arr = vec![Cell::new_int(0); num_cells];
    let mut formula_array = vec![Formula::default(); num_cells];
    let mut graph = Graph::new(num_cells);
    let mut state = State::new();
    let mut undo_stack: Vec<StateSnapshot> = Vec::new();
    let mut redo_stack: Vec<StateSnapshot> = Vec::new();

    // Create the extended state with all components
    let extended_state = ExtendedState {
        sheet: sheet.clone(),
        cells: cells.clone(),
        formula_array: formula_array.clone(),
        graph: graph.clone(),
        state: state.clone(),
        undo_stack,
        redo_stack,
        current_x: 0,
        current_y: 0,
    };
    let app_state = Arc::new(RwLock::new(extended_state));

    // Create a CORS layer that allows any origin
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Add the CORS layer to your router
    let app = Router::new()
        .route("/sheet", get(get_sheet))
        .route("/update-cell", post(update_cell))
        .route("/api/query", post(process_query))
        .route("/api/undo", post(undo_action)) // New endpoint for undo
        .route("/api/redo", post(redo_action)) // New endpoint for redo
        .with_state(app_state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("✅ Server running at http://{}", addr);
    println!(
        "🔄 Cell update endpoint available at http://{}/update-cell",
        addr
    );
    println!("📝 Query endpoint available at http://{}/api/query", addr);
    println!("↩️ Undo endpoint available at http://{}/api/undo", addr);
    println!("↪️ Redo endpoint available at http://{}/api/redo", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
