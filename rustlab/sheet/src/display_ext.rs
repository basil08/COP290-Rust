use crate::function_ext::{Cell, CellValue};
use crate::graph_ext::Graph;
use crate::parser_ext::cell_parser;
use std::cmp::min;

/// Prints a 10×10 view of the spreadsheet starting from the given coordinates.
///
/// # Arguments
/// * `currx` - The starting column index (0-based) for the visible area.
/// * `curry` - The starting row index (0-based) for the visible area.
/// * `arr` - The flat array of [`Cell`] objects representing the spreadsheet.
/// * `c` - Total number of columns in the spreadsheet.
/// * `r` - Total number of rows in the spreadsheet.
///
/// The function prints a tabular view of up to 10 rows and columns. Column headers are displayed
/// in Excel-style format (A, B, ..., Z, AA, AB, ...), and invalid cells show `ERR`.
///
/// This function is used to simulate viewport-like scrolling in a terminal-based spreadsheet UI.
pub fn printer(currx: i32, curry: i32, arr: &[Cell], c: i32, r: i32) {
    print!("      ");

    let display_cols = min(10, c - currx);
    for i in 0..display_cols {
        let mut val = currx + i + 1;
        let mut chars: Vec<char> = Vec::new();
        while val > 0 {
            val -= 1;
            chars.push((b'A' + (val % 26) as u8) as char);
            val /= 26;
        }
        let header: String = chars.into_iter().rev().collect();
        print!("{:<10}", header);
    }
    println!();

    let display_rows = min(10, r - curry);
    for j in 0..display_rows {
        print!("{:<3}   ", curry + j + 1);
        for i in 0..display_cols {
            let value = &arr[((currx + i) + c * (curry + j)) as usize];
            if !value.is_valid {
                print!("{:<10}", "ERR");
            } else {
                match &value.value {
                    CellValue::Int(i) => print!("{:<10}", i),
                    CellValue::Float(f) => print!("{:<10.2}", f),
                    CellValue::String(s) => {
                        let truncated = if s.len() > 10 {
                            format!("{:.10}", &s[..10])
                        } else {
                            s.clone()
                        };
                        print!("{:<10}", truncated);
                    }
                }
            }
        }
        println!();
    }
}

/// Handles scrolling logic for the spreadsheet viewport using directional commands or `scroll_to`.
///
/// # Arguments
/// * `a` - Scroll command. Can be:
///   - `"w"` - scroll up
///   - `"s"` - scroll down
///   - `"a"` - scroll left
///   - `"d"` - scroll right
///   - `"scroll_to <cell>"` - jump to a specific cell like `"scroll_to A1"`
/// * `_arr` - Unused but passed in case of future validation or redraw triggers.
/// * `currx` - Mutable reference to the current top-left column of the viewport.
/// * `curry` - Mutable reference to the current top-left row of the viewport.
/// * `c` - Total number of columns in the spreadsheet.
/// * `r` - Total number of rows in the spreadsheet.
/// * `_graph` - Currently unused, but may be used for recalculation dependencies or context.
///
/// # Returns
/// * `Ok(())` if scroll was successful or allowed.
/// * `Err(...)` if an unknown or malformed scroll command is passed.
///
/// This function updates `currx` and `curry` based on navigation input, ensuring they remain in bounds.
pub fn scroller(
    a: &str,
    _arr: &[Cell],
    currx: &mut i32,
    curry: &mut i32,
    c: i32,
    r: i32,
    _graph: &Graph,
) -> Result<(), &'static str> {
    // let mut _flag = false;

    match a {
        "w" => {
            if *curry < 10 {
                if *curry > 0 {
                    *curry = 0;
                } else {
                    // flag = true;
                }
            } else {
                *curry -= 10;
            }
        }
        "d" => {
            let remaining_cols = c - *currx - 10;
            if remaining_cols <= 0 {
                // flag = true;
            } else if remaining_cols < 10 {
                *currx += remaining_cols;
            } else {
                *currx += 10;
            }
        }
        "a" => {
            if *currx < 10 {
                if *currx > 0 {
                    *currx = 0;
                } else {
                    // flag = true;
                }
            } else {
                *currx -= 10;
            }
        }
        "s" => {
            let remaining_rows = r - *curry - 10;
            if remaining_rows <= 0 {
                // flag = true;
            } else if remaining_rows < 10 {
                *curry += remaining_rows;
            } else {
                *curry += 10;
            }
        }
        s if s.starts_with("scroll_to ") => {
            let parts: Vec<&str> = s[9..].split_whitespace().collect();
            if parts.len() != 1 {
                return Err("Invalid scroll_to format");
            }
            let cell = cell_parser(parts[0], c, r, 0, parts[0].len() - 1)?;
            if cell < 0 || cell >= (c * r) {
                // flag = true;
            } else {
                let start_row = cell / c;
                let start_col = cell % c;
                if start_row >= r || start_col >= c {
                    // flag = true;
                } else {
                    *currx = start_col;
                    *curry = start_row;
                }
            }
        }
        _ => return Err("Unknown scroll command"),
    }

    Ok(())
}
