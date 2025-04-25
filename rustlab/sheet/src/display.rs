use crate::graph::Graph;
use std::cmp::min;
// use std::i32;

/// Prints a 10×10 viewport of the spreadsheet from the current scroll position.
///
/// This version assumes the spreadsheet is stored as a flat `i32` array, where each cell is either a valid integer
/// or `i32::MIN` to indicate an invalid cell.
///
/// # Arguments
/// * `currx` - Current horizontal scroll offset (column index).
/// * `curry` - Current vertical scroll offset (row index).
/// * `arr` - Flat array of spreadsheet values (row-major order).
/// * `c` - Total number of columns in the spreadsheet.
/// * `r` - Total number of rows in the spreadsheet.
///
/// The function prints:
/// - Column headers in Excel-style format (`A`, `B`, ..., `Z`, `AA`, `AB`, etc.).
/// - Row numbers along the left edge.
/// - Values from the spreadsheet grid, with `"ERR"` shown for invalid entries.
pub fn printer(currx: usize, curry: usize, arr: &[i32], c: usize, r: usize) {
    print!("      ");

    let visible_cols = min(10, c.saturating_sub(currx));
    for i in 0..visible_cols {
        let mut val = currx + i + 1;
        let mut s = Vec::new();

        // Convert column number to Excel-style letter (e.g., 1 -> A, 27 -> AA)
        while val > 0 && s.len() < 6 {
            val -= 1;
            s.push((b'A' + (val % 26) as u8) as char);
            val /= 26;
        }
        s.reverse();
        print!("{:<10}", s.iter().collect::<String>());
    }
    println!();

    let visible_rows = min(10, r.saturating_sub(curry));
    for j in 0..visible_rows {
        print!("{:<3}   ", curry + j + 1); // Print row number

        for i in 0..visible_cols {
            let idx = (currx + i) + c * (curry + j);
            let value = arr[idx];

            if value == i32::MIN {
                print!("{:<10}", "ERR");
            } else {
                print!("{:<10}", value);
            }
        }
        println!();
    }
}

/// Scrolls the spreadsheet viewport based on user input.
///
/// Accepts basic WASD-style controls for navigating:
/// - `"w"`: Scroll up
/// - `"a"`: Scroll left
/// - `"s"`: Scroll down
/// - `"d"`: Scroll right
///
/// # Arguments
/// * `a` - Scroll command (`"w"`, `"a"`, `"s"`, `"d"`).
/// * `_arr` - Spreadsheet values (not used in current implementation).
/// * `currx` - Mutable reference to current horizontal offset.
/// * `curry` - Mutable reference to current vertical offset.
/// * `c` - Total number of columns in the spreadsheet.
/// * `r` - Total number of rows in the spreadsheet.
/// * `_graph` - Reference to graph (not used, reserved for future dependency tracking).
///
/// # Returns
/// * `0` on successful scroll or no movement (already at boundary).
/// * `-1` if the command is unrecognized.
pub fn scroller(
    a: &str,
    _arr: &mut [i32],
    currx: &mut usize,
    curry: &mut usize,
    c: usize,
    r: usize,
    _graph: &mut Graph,
) -> i32 {
    match a {
        "w" => {
            if *curry < 10 {
                if *curry > 0 {
                    *curry = 0;
                } else {
                    return 0;
                }
            } else {
                *curry -= 10;
            }
        }
        "a" => {
            if *currx < 10 {
                if *currx > 0 {
                    *currx = 0;
                } else {
                    return 0;
                }
            } else {
                *currx -= 10;
            }
        }
        "s" => {
            let remaining_rows = r.saturating_sub(*curry + 10);
            if remaining_rows == 0 {
                return 0;
            } else {
                *curry += min(10, remaining_rows);
            }
        }
        "d" => {
            let remaining_cols = c.saturating_sub(*currx + 10);
            if remaining_cols == 0 {
                return 0;
            } else {
                *currx += min(10, remaining_cols);
            }
        }
        _ if a.starts_with("scroll_to ") => {
            if let Some(cell) = crate::parser::cell_parser(a, c, r, 10, a.len() - 1, _graph) {
                let row = cell / c;
                let col = cell % c;

                if row < r && col < c {
                    *currx = col;
                    *curry = row;
                } else {
                    return -1;
                }
            } else {
                return -1;
            }
        }
        _ => return -1, // Unknown command
    }

    0
}
