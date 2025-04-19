// main.rs
mod display;
mod function;
mod graph;
mod parser;


use std::env;
use std::io::{self, Write};
use std::time::Instant;

use graph::{Graph, Formula};

static mut NUM_CELLS: usize = 0;
static mut HAS_CYCLE: bool = false;
static mut INVALID_RANGE: bool = false;

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <rows> <columns>", args[0]);
        return;
    }

    let r: usize = args[1].parse().expect("Invalid row value");
    let c: usize = args[2].parse().expect("Invalid column value");

    let num_cells = r * c;
    unsafe { NUM_CELLS = num_cells; }

    let mut graph = Graph::new(num_cells);
    let mut formula_array = vec![Formula::default(); num_cells];
    let mut arr = vec![0; r * c];

    let mut currx = 0;
    let mut curry = 0;
    let mut output_disabled = false;

    if !output_disabled {
        display::printer(currx, curry, &arr, c, r);
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
                display::printer(currx, curry, &arr, c, r);
                print!("[{:.6}] (ok) ", loop_start.elapsed().as_secs_f64());
                continue;
            }
            _ => {
                if ['w', 'a', 's', 'd'].contains(&trimmed.chars().next().unwrap_or(' ')) || trimmed.starts_with("scroll_to ") {
                    if display::scroller(trimmed, &mut arr, &mut currx, &mut curry, c, r, &mut graph) == -1 {
                        status = -1;
                    }
                } else {
                    status = parser::parser(trimmed, c, r, &mut arr, &mut graph, &mut formula_array);
                }
            }
        }

        if !output_disabled {
            display::printer(currx, curry, &arr, c, r);
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
