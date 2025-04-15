use std::io::{self, Write};
use std::env;
use std::time::Instant;
use graph::StateSnapshot;


mod display;
mod function;
mod graph;
mod parser;
mod util;

use display::{printer, scroller};
use function::Cell;
use graph::{Graph, Formula, State};
use parser::parser;
fn create_snapshot(
    arr: &Vec<Cell>,
    formula_array: &Vec<Formula>,
    graph: &Graph,
) -> StateSnapshot {
    StateSnapshot {
        arr: arr.clone(),
        formula_array: formula_array.clone(),
        graph: graph.clone(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: ./sheet <rows> <cols>");
        std::process::exit(1);
    }
    let mut undo_stack: Vec<StateSnapshot> = Vec::new();
let mut redo_stack: Vec<StateSnapshot> = Vec::new();

    let rows: usize = args[1].parse().expect("Invalid number of rows");
    let cols: usize = args[2].parse().expect("Invalid number of columns");
    let num_cells = cols * rows;
    let cols_i32: i32 = cols.try_into().unwrap();
    let rows_i32: i32 = rows.try_into().unwrap();

    let mut arr = vec![Cell::new_int(0); num_cells];
    let mut formula_array = vec![Formula { op_type: 0, op_info1: -1, op_info2: -1 }; num_cells];
    let mut graph = Graph::new(num_cells);
    let mut state = State::new();
    let mut currx = 0;
    let mut curry = 0;
    let mut output_enabled = true;

    if output_enabled {
        printer(currx, curry, &arr, cols_i32, rows_i32);
    }

    loop {
        let start = Instant::now();
        print!("[0.0] (ok) > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "q" {
            break;
        }

        let result = match input {
            "disable_output" => {
                output_enabled = false;
                Ok(())
            }
            "enable_output" => {
                output_enabled = true;
                Ok(())
            }
            "w" | "a" | "s" | "d" => {
                scroller(input, &arr, &mut currx, &mut curry, cols_i32, rows_i32, &graph)
            }
            _ if input.starts_with("scroll_to ") => {
                let cmd = input.replacen("scroll_to ", "", 1);
                scroller(&format!("scroll_to {}", cmd), &arr, &mut currx, &mut curry, cols_i32, rows_i32, &graph)
            }
            "undo" => {
                if let Some(prev) = undo_stack.pop() {
                    redo_stack.push(create_snapshot(&arr, &formula_array, &graph));
                    arr = prev.arr;
                    formula_array = prev.formula_array;
                    graph = prev.graph;
                    Ok(())
                } else {
                    Err("Nothing to undo")
                }
            }
            "redo" => {
                if let Some(next) = redo_stack.pop() {
                    undo_stack.push(create_snapshot(&arr, &formula_array, &graph));
                    arr = next.arr;
                    formula_array = next.formula_array;
                    graph = next.graph;
                    Ok(())
                } else {
                    Err("Nothing to redo")
                }
            }
            _ => {
                // 🧠 Save snapshot before parsing a new command
                undo_stack.push(create_snapshot(&arr, &formula_array, &graph));
                if undo_stack.len() > 5 {
                    undo_stack.remove(0);
                }
                redo_stack.clear(); // New action invalidates redo history
        
                parser(input, cols_i32, rows_i32, &mut arr, &mut graph, &mut formula_array, &mut state)
            }
        };
        
        let elapsed = start.elapsed().as_secs_f32();
        match result {
            Ok(_) => {
                if output_enabled {
                    printer(currx, curry, &arr, cols_i32, rows_i32);
                }
                println!("[{:.1}] (ok) >", elapsed);
            }
            Err(e) => {
                if output_enabled {
                    printer(currx, curry, &arr, cols_i32, rows_i32);
                }
                println!("[{:.1}] ({}) >", elapsed, e);
            }
        }
    }

    Ok(())
}
