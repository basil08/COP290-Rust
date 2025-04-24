use sheet::display::{printer, scroller};
use sheet::graph::Graph;
use std::i32;

fn generate_grid_with_pattern(c: usize, r: usize, err_at: Option<usize>) -> Vec<i32> {
    let mut grid = vec![0; c * r];
    if let Some(idx) = err_at {
        grid[idx] = i32::MIN;
    }
    for i in 0..(c * r) {
        if Some(i) != err_at {
            grid[i] = i as i32;
        }
    }
    grid
}

#[test]
fn test_printer_draws_headers_and_rows() {
    let c = 12;
    let r = 12;
    let grid = generate_grid_with_pattern(c, r, None);
    // This won't validate stdout, but ensures full execution path
    printer(0, 0, &grid, c, r); // draws A-L and 1–10
    printer(2, 3, &grid, c, r); // draws C–L and 4–13 (max 10 visible)
}

#[test]
fn test_printer_shows_err_cell() {
    let c = 10;
    let r = 10;
    let idx = 5 + 3 * c; // cell at col=5, row=3
    let grid = generate_grid_with_pattern(c, r, Some(idx));
    printer(0, 0, &grid, c, r); // triggers ERR printing path
}

#[test]
fn test_scroller_top_left_no_move() {
    let mut x = 0;
    let mut y = 0;
    let c = 10;
    let r = 10;
    let mut arr = vec![0; c * r];
    let mut graph = Graph::new(c * r);

    assert_eq!(scroller("w", &mut arr, &mut x, &mut y, c, r, &mut graph), 0);
    assert_eq!(scroller("a", &mut arr, &mut x, &mut y, c, r, &mut graph), 0);
}

#[test]
fn test_scroller_down_and_right_partial_and_full_jump() {
    let mut x = 0;
    let mut y = 0;
    let c = 15;
    let r = 13;
    let mut arr = vec![0; c * r];
    let mut graph = Graph::new(c * r);

    assert_eq!(scroller("s", &mut arr, &mut x, &mut y, c, r, &mut graph), 0);
    assert!(y > 0);
    assert_eq!(scroller("d", &mut arr, &mut x, &mut y, c, r, &mut graph), 0);
    assert!(x > 0);
}

#[test]
fn test_scroller_invalid_command() {
    let mut x = 0;
    let mut y = 0;
    let c = 10;
    let r = 10;
    let mut arr = vec![0; c * r];
    let mut graph = Graph::new(c * r);

    assert_eq!(
        scroller(
            "invalid_command",
            &mut arr,
            &mut x,
            &mut y,
            c,
            r,
            &mut graph
        ),
        -1
    );
}
