// display.rs

use crate::graph::Graph;
use std::cmp::{max, min};
use std::i32;

pub fn printer(currx: usize, curry: usize, arr: &[i32], c: usize, r: usize) {
    print!("      ");

    let visible_cols = min(10, c.saturating_sub(currx));
    for i in 0..visible_cols {
        let mut val = currx + i + 1;
        let mut s = Vec::new();

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
        print!("{:<3}   ", curry + j + 1);
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

pub fn scroller(
    a: &str,
    _arr: &mut [i32],
    currx: &mut usize,
    curry: &mut usize,
    c: usize,
    r: usize,
    graph: &mut Graph,
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
            if let Some(cell) = crate::parser::cell_parser(a, c, r, 10, a.len() - 1, graph) {
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
        _ => return -1,
    }

    0
}
