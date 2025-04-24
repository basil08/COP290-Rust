use std::collections::VecDeque;
use std::i32;
/// Represents a formula associated with a spreadsheet cell.
///
/// - `op_type`: Indicates the operation type (e.g. 0: assign, 1-4: unary op, 5-8: binary op, 9-13: range functions).
/// - `op_info1`: First operand (may be a cell index or constant).
/// - `op_info2`: Second operand or auxiliary value.
#[derive(Clone, Copy, Debug, Default)]
pub struct Formula {
    pub op_type: i32,
    pub op_info1: i32,
    pub op_info2: i32,
}

/// A node in an adjacency list used to represent dependencies between cells.
#[derive(Debug)]
pub struct Cell {
    /// Index of the dependent cell.
    pub cell: usize,
    /// Pointer to the next dependent cell.
    pub next: Option<Box<Cell>>,
}

/// Represents a rectangular cell range with a dependent output cell.
#[derive(Debug)]
pub struct Range {
    /// Start cell of the range.
    pub start_cell: usize,
    /// End cell of the range.
    pub end_cell: usize,
    /// Cell that depends on the result of the range.
    pub dependent_cell: usize,
    /// Pointer to the next range node.
    pub next: Option<Box<Range>>,
}

/// The core graph structure used for tracking dependencies and formula evaluation.
pub struct Graph {
    /// Adjacency list where each index points to a list of dependents.
    pub adj_lists: Vec<Option<Box<Cell>>>,
    /// Linked list of rectangular ranges with dependent cells.
    pub ranges: Option<Box<Range>>,
    /// Total number of cells in the spreadsheet.
    pub num_cells: usize,
}

impl Graph {
    /// Creates a new `Graph` with the given number of cells.
    pub fn new(num_cells: usize) -> Self {
        Self {
            adj_lists: vec![None; num_cells],
            ranges: None,
            num_cells,
        }
    }
    /// Returns a new boxed `Cell` node for the given index.
    pub fn add_cell(cell: usize) -> Option<Box<Cell>> {
        Some(Box::new(Cell { cell, next: None }))
    }
    /// Returns a new boxed `Range` from start to end affecting the dependent cell.

    pub fn add_range(start: usize, end: usize, dependent: usize) -> Option<Box<Range>> {
        Some(Box::new(Range {
            start_cell: start,
            end_cell: end,
            dependent_cell: dependent,
            next: None,
        }))
    }
    /// Adds a directed edge from one cell to another.

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if self.has_edge(from, to) {
            return;
        }
        let new_cell = Box::new(Cell {
            cell: to,
            next: self.adj_lists[from].take(),
        });
        self.adj_lists[from] = Some(new_cell);
    }
    /// Checks whether a directed edge already exists from one cell to another.

    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        let mut current = &self.adj_lists[from];
        while let Some(cell) = current {
            if cell.cell == to {
                return true;
            }
            current = &cell.next;
        }
        false
    }
    /// Deletes a dependency edge from the graph.

    pub fn delete_edge(&mut self, from: usize, to: usize) {
        let mut head = self.adj_lists[from].take();
        let mut dummy = Box::new(Cell {
            cell: 0,
            next: head,
        });
        let mut prev = &mut dummy;

        while let Some(mut node) = prev.next.take() {
            if node.cell == to {
                prev.next = node.next.take();
                break;
            } else {
                prev.next = Some(node);
                prev = prev.next.as_mut().unwrap();
            }
        }

        self.adj_lists[from] = dummy.next;
    }
    /// Inserts a new range-based dependency into the graph.

    pub fn add_range_to_graph(&mut self, start: usize, end: usize, dependent: usize) {
        let mut new_range = Self::add_range(start, end, dependent);
        if let Some(ref mut r) = new_range {
            r.next = self.ranges.take();
        }
        self.ranges = new_range;
    }
    /// Removes a range from the graph if it targets the specified dependent cell.

    pub fn delete_range(&mut self, dependent: usize) {
        let mut prev: *mut Option<Box<Range>> = &mut self.ranges;
        unsafe {
            while let Some(ref mut current) = *prev {
                if current.dependent_cell == dependent {
                    *prev = current.next.take();
                } else {
                    prev = &mut current.next;
                }
            }
        }
    }
    /// Adds a formula to the formula array for a specific cell.

    pub fn add_formula(
        graph: &mut Graph,
        cell: usize,
        c1: usize,
        c2: usize,
        op_type: i32,
        formula_array: &mut [Formula],
    ) {
        formula_array[cell] = Formula {
            op_type,
            op_info1: c1 as i32,
            op_info2: c2 as i32,
        };
    }
    /// Evaluates two integers with the specified arithmetic operation.

    pub fn arithmetic_eval2(v1: i32, v2: i32, op: char) -> i32 {
        match op {
            '+' => v1 + v2,
            '-' => v1 - v2,
            '*' => v1 * v2,
            '/' if v2 != 0 => v1 / v2,
            _ => i32::MIN,
        }
    }
    /// Performs a depth-first topological sort starting from a cell, detecting cycles.
    ///
    /// Updates `stack` with a valid evaluation order if no cycles are found.

    pub fn topo_sort_from_cell(
        &self,
        start: usize,
        cols: usize,
        visited: &mut Vec<bool>,
        on_stack: &mut Vec<bool>,
        stack: &mut Vec<usize>,
        formula_array: &[Formula],
        has_cycle: &mut bool,
    ) {
        if *has_cycle {
            return;
        }

        visited[start] = true;
        on_stack[start] = true;

        let mut current = &self.adj_lists[start];
        while let Some(node) = current {
            let dep = node.cell;
            if !visited[dep] {
                self.topo_sort_from_cell(
                    dep,
                    cols,
                    visited,
                    on_stack,
                    stack,
                    formula_array,
                    has_cycle,
                );
            } else if on_stack[dep] {
                *has_cycle = true;
                return;
            }
            current = &node.next;
        }

        let mut range = &self.ranges;
        while let Some(r) = range {
            let sr = r.start_cell / cols;
            let sc = r.start_cell % cols;
            let er = r.end_cell / cols;
            let ec = r.end_cell % cols;
            let row = start / cols;
            let col = start % cols;

            if row >= sr && row <= er && col >= sc && col <= ec {
                if !visited[r.dependent_cell] {
                    self.topo_sort_from_cell(
                        r.dependent_cell,
                        cols,
                        visited,
                        on_stack,
                        stack,
                        formula_array,
                        has_cycle,
                    );
                } else if on_stack[r.dependent_cell] {
                    *has_cycle = true;
                    return;
                }
            }

            range = &r.next;
        }

        on_stack[start] = false;
        stack.push(start);
    }
    /// Evaluates and updates all dependent cells starting from `start_cell`.
    ///
    /// Performs topological sort, evaluates formulas, handles errors and propagation.

    pub fn recalc(
        &mut self,
        cols: usize,
        arr: &mut [i32],
        start_cell: usize,
        formula_array: &mut [Formula],
        has_cycle: &mut bool,
    ) {
        let mut visited = vec![false; self.num_cells];
        let mut on_stack = vec![false; self.num_cells];
        let mut stack = Vec::new();

        self.topo_sort_from_cell(
            start_cell,
            cols,
            &mut visited,
            &mut on_stack,
            &mut stack,
            formula_array,
            has_cycle,
        );

        if *has_cycle {
            return;
        }

        while let Some(cell) = stack.pop() {
            let formula = formula_array[cell];
            match formula.op_type {
                0 => {
                    arr[cell] = formula.op_info1;
                }

                1..=4 => {
                    let v1 = arr[formula.op_info1 as usize];
                    let v2 = formula.op_info2;
                    if v1 == i32::MIN || (formula.op_type == 4 && v2 == 0) {
                        arr[cell] = i32::MIN;
                    } else {
                        let op = match formula.op_type {
                            1 => '+',
                            2 => '-',
                            3 => '*',
                            4 => '/',
                            _ => '?',
                        };
                        arr[cell] = Graph::arithmetic_eval2(v1, v2, op);
                    }
                }

                5..=8 => {
                    let v1 = arr[formula.op_info1 as usize];
                    let v2 = arr[formula.op_info2 as usize];
                    if v1 == i32::MIN || v2 == i32::MIN || (formula.op_type == 8 && v2 == 0) {
                        arr[cell] = i32::MIN;
                    } else {
                        let op = match formula.op_type {
                            5 => '+',
                            6 => '-',
                            7 => '*',
                            8 => '/',
                            _ => '?',
                        };
                        arr[cell] = Graph::arithmetic_eval2(v1, v2, op);
                    }
                }

                9..=13 => {
                    let start_cell = formula.op_info1 as usize;
                    let end_cell = formula.op_info2 as usize;

                    let start_row = start_cell / cols;
                    let start_col = start_cell % cols;
                    let end_row = end_cell / cols;
                    let end_col = end_cell % cols;

                    // Optimize by swapping if start > end
                    let (start_row, end_row) = if start_row > end_row {
                        (end_row, start_row)
                    } else {
                        (start_row, end_row)
                    };
                    let (start_col, end_col) = if start_col > end_col {
                        (end_col, start_col)
                    } else {
                        (start_col, end_col)
                    };

                    let mut sum = 0;
                    let mut count = 0;
                    let mut min_val = i32::MAX;
                    let mut max_val = i32::MIN;
                    let mut has_error = false;
                    let mut values = vec![];

                    // First pass: Calculate sum, min, max and check for errors
                    for row in start_row..=end_row {
                        for col in start_col..=end_col {
                            let idx = row * cols + col;
                            let val = arr[idx];

                            if val == i32::MIN {
                                has_error = true;
                                break;
                            }

                            sum += val;
                            count += 1;
                            min_val = min_val.min(val);
                            max_val = max_val.max(val);
                            values.push(val);
                        }
                        if has_error {
                            break;
                        }
                    }

                    if has_error || count == 0 {
                        arr[cell] = i32::MIN;
                    } else {
                        arr[cell] = match formula.op_type {
                            9 => min_val,      // MIN
                            10 => max_val,     // MAX
                            11 => sum / count, // AVG
                            12 => sum,         // SUM
                            13 => {
                                // STDEV
                                let mean = sum as f64 / count as f64;
                                let variance = values
                                    .iter()
                                    .map(|&x| {
                                        let diff = x as f64 - mean;
                                        diff * diff
                                    })
                                    .sum::<f64>()
                                    / count as f64;
                                variance.sqrt().round() as i32
                            }
                            _ => i32::MIN,
                        };
                    }
                }
                14 => {
                    let v = if formula.op_info1 as usize == cell {
                        formula.op_info2
                    } else {
                        arr[formula.op_info1 as usize]
                    };

                    if v == i32::MIN {
                        arr[cell] = i32::MIN;
                    } else {
                        if v > 0 {
                            // println!("Sleeping for {} seconds 3", v);
                            std::thread::sleep(std::time::Duration::from_secs(v as u64));
                        }
                        arr[cell] = v;
                    }
                }

                15 => {
                    let v1 = formula.op_info1;
                    let v2 = arr[formula.op_info2 as usize];
                    if v2 == i32::MIN || (v2 == 0 && formula.op_type == 4) {
                        arr[cell] = i32::MIN;
                    } else {
                        let op = '/'; // op_type 15 is used for CONSTANT / CELL
                        arr[cell] = Graph::arithmetic_eval2(v1, v2, op);
                    }
                }

                _ => {
                    arr[cell] = i32::MIN;
                }
            }
        }
    }
}
/// Clones a dependency list node recursively.
impl Clone for Cell {
    fn clone(&self) -> Self {
        Self {
            cell: self.cell,
            next: self.next.clone(),
        }
    }
}
/// Clones a range list node recursively.
impl Clone for Range {
    fn clone(&self) -> Self {
        Self {
            start_cell: self.start_cell,
            end_cell: self.end_cell,
            dependent_cell: self.dependent_cell,
            next: self.next.clone(),
        }
    }
}
