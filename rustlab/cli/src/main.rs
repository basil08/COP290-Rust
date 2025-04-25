use std::env;
use std::io::{self, Write};
use std::time::Instant;

// Import everything upfront
use sheet::function_ext;
use sheet::graph_ext::{self, StateSnapshot};

static mut NUM_CELLS: usize = 0;
static mut HAS_CYCLE: bool = false;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut extended = false;
    let mut r = 0;
    let mut c = 0;

    for arg in &args[1..] {
        if arg == "-extended" {
            extended = true;
        } else if r == 0 {
            r = arg.parse().unwrap_or(0);
        } else if c == 0 {
            c = arg.parse().unwrap_or(0);
        }
    }

    if r == 0 || c == 0 {
        println!("Usage: {} <rows> <columns> [-extended]", args[0]);
        return Ok(());
    }

    if extended {
        run_extended(r, c)?;
    } else {
        run_standard(r, c);
    }

    Ok(())
}

// ✅ Snapshot function just for extended mode
fn create_snapshot_extended(
    arr: &[function_ext::Cell],
    formula_array: &[graph_ext::Formula],
    graph: &graph_ext::Graph,
) -> graph_ext::StateSnapshot {
    graph_ext::StateSnapshot {
        arr: arr.to_owned(),
        formula_array: formula_array.to_owned(),
        graph: graph.clone(),
    }
}

// ✅ Standard mode logic (unchanged)
fn run_standard(r: usize, c: usize) {
    use sheet::display::{printer, scroller};
    use sheet::function::*;
    use sheet::graph::{Formula, Graph};
    use sheet::parser::parser;

    let start = Instant::now();
    let num_cells = r * c;
    unsafe {
        NUM_CELLS = num_cells;
    }

    let mut graph = Graph::new(num_cells);
    let mut formula_array = vec![Formula::default(); num_cells];
    let mut arr = vec![0; num_cells];

    let mut currx = 0;
    let mut curry = 0;
    let mut output_disabled = false;

    if !output_disabled {
        printer(currx, curry, &arr, c, r);
    }

    print!("[{:.6}] (ok) ", start.elapsed().as_secs_f64());

    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if trimmed == "q" {
            break;
        }

        let loop_start = Instant::now();
        let mut status = 1;

        match trimmed {
            "disable_output" => {
                output_disabled = true;
                print!("[{:.6}] (ok) ", loop_start.elapsed().as_secs_f64());
                continue;
            }
            "enable_output" => {
                output_disabled = false;
                printer(currx, curry, &arr, c, r);
                print!("[{:.6}] (ok) ", loop_start.elapsed().as_secs_f64());
                continue;
            }
            _ => {
                if ['w', 'a', 's', 'd'].contains(&trimmed.chars().next().unwrap_or(' '))
                    || trimmed.starts_with("scroll_to ")
                {
                    if scroller(trimmed, &mut arr, &mut currx, &mut curry, c, r, &mut graph) == -1 {
                        status = -1;
                    }
                } else {
                    status = parser(trimmed, c, r, &mut arr, &mut graph, &mut formula_array[..]);
                }
            }
        }

        if !output_disabled {
            printer(currx, curry, &arr, c, r);
        }

        let elapsed = loop_start.elapsed().as_secs_f64();
        unsafe {
            if status > 0 {
                print!("[{:.6}] (ok) ", elapsed);
            } else if HAS_CYCLE {
                print!("[{:.6}] (Circular dependency detected) ", elapsed);
                HAS_CYCLE = false;
            } else if INVALID_RANGE {
                print!("[{:.6}] (Invalid range) ", elapsed);
                INVALID_RANGE = false;
            } else {
                print!("[{:.6}] (unrecognized command) ", elapsed);
            }
        }
    }
}

// ✅ Extended mode logic with undo/redo and cell types
fn run_extended(r: usize, c: usize) -> Result<(), Box<dyn std::error::Error>> {
    use sheet::display_ext::{printer, scroller};
    use sheet::function_ext::Cell;
    use sheet::graph_ext::{Formula, Graph, State};
    use sheet::parser_ext::parser;

    let num_cells = r * c;
    let cols_i32 = c as i32;
    let rows_i32 = r as i32;

    let mut arr = vec![Cell::new_int(0); num_cells];
    let mut formula_array = vec![Formula::default(); num_cells];
    let mut graph = Graph::new(num_cells);
    let mut state = State::new();
    let mut undo_stack: Vec<StateSnapshot> = Vec::new();
    let mut redo_stack: Vec<StateSnapshot> = Vec::new();
    let mut currx = 0;
    let mut curry = 0;
    let mut output_enabled = true;

    if output_enabled {
        printer(currx, curry, &arr, cols_i32, rows_i32);
    }
    print!("[0.0] (ok) > ");
    loop {
        let start = Instant::now();

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
                scroller(
                    &format!("scroll_to {}", cmd),
                    &arr,
                    &mut currx,
                    &mut curry,
                    cols_i32,
                    rows_i32,
                    &graph,
                )
            }
            "undo" => {
                if let Some(prev) = undo_stack.pop() {
                    redo_stack.push(create_snapshot_extended(&arr, &formula_array, &graph));
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
                    undo_stack.push(create_snapshot_extended(&arr, &formula_array, &graph));
                    arr = next.arr;
                    formula_array = next.formula_array;
                    graph = next.graph;
                    Ok(())
                } else {
                    Err("Nothing to redo")
                }
            }
            _ => {
                undo_stack.push(create_snapshot_extended(&arr, &formula_array, &graph));
                if undo_stack.len() > 5 {
                    undo_stack.remove(0);
                }
                redo_stack.clear();
                parser(
                    input,
                    cols_i32,
                    rows_i32,
                    &mut arr,
                    &mut graph,
                    &mut formula_array,
                    &mut state,
                )
            }
        };

        let elapsed = start.elapsed().as_secs_f32();
        match result {
            Ok(_) => {
                if output_enabled {
                    printer(currx, curry, &arr, cols_i32, rows_i32);
                }
                print!("[{:.1}] (ok) > ", elapsed);
            }
            Err(e) => {
                if output_enabled {
                    printer(currx, curry, &arr, cols_i32, rows_i32);
                }
                print!("[{:.1}] ({}) > ", elapsed, e);
            }
        }
    }

    Ok(())
}
