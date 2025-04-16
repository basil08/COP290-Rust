use std::cmp::min;
use crate::function::{Cell, CellValue};
use crate::parser::cell_parser;
use crate::graph::Graph;

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
                        // Truncate the string if it's longer than 10 characters
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

pub fn scroller(a: &str, _arr: &[Cell], currx: &mut i32, curry: &mut i32, c: i32, r: i32, _graph: &Graph) -> Result<(), &'static str> {
    let mut flag = false;
    
    match a {
        "w" => {
            if *curry < 10 {
                if *curry > 0 {
                    *curry = 0;
                } else {
                    flag = true;
                }
            } else {
                *curry -= 10;
            }
        }
        "d" => {
            let remaining_cols = c - *currx - 10;
            if remaining_cols <= 0 {
                flag = true;
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
                    flag = true;
                }
            } else {
                *currx -= 10;
            }
        }
        "s" => {
            let remaining_rows = r - *curry - 10;
            if remaining_rows <= 0 {
                flag = true;
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
                flag = true;
            } else {
                let start_row = cell / c;
                let start_col = cell % c;
                if start_row >= r || start_col >= c {
                    flag = true;
                } else {
                    *currx = start_col;
                    *curry = start_row;
                }
            }
        }
        _ => return Err("Unknown scroll command"),
    }

    if flag {
        // Invalid scroll, no change
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    #[test]
    fn test_display() {
        let mut graph = Graph::new(100);
        let arr = vec![Cell::new_int(0); 100];
        printer(0, 0, &arr, 10, 10);
        
        let mut currx = 0;
        let mut curry = 0;
        assert!(scroller("d", &arr, &mut currx, &mut curry, 10, 10, &graph).is_ok());
        assert_eq!(currx, 10);
        assert_eq!(curry, 0);
    }
}