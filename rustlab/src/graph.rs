use std::cmp::{max, min};
use std::thread::sleep;
use std::time::Duration;

// Constants
pub static mut NUM_CELLS: usize = 0;
static mut HAS_CYCLE: bool = false;

// Formula structure
#[derive(Clone, Copy)]
pub struct Formula {
    pub op_type: i32,
    pub op_info1: i32,
    pub op_info2: i32,
}

// Cell structure for adjacency list
pub struct Cell {
    pub cell: i32,
    pub next: Option<Box<Cell>>,
}

// Range structure to store range endpoints
pub struct Range {
    pub start_cell: i32,
    pub end_cell: i32,
    pub dependent_cell: i32,
    pub next: Option<Box<Range>>,
}

// Graph structure
pub struct Graph {
    pub adj_lists_head: Vec<Option<Box<Cell>>>,
    pub ranges_head: Option<Box<Range>>,
}

// Utility functions
fn min2(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn max2(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn arithmetic_eval2(v1: i32, v2: i32, op: char) -> i32 {
    match op {
        '+' => v1 + v2,
        '-' => v1 - v2,
        '*' => v1 * v2,
        '/' => v1 / v2,
        _ => i32::MIN,
    }
}


impl Graph {
    // Create a new Graph
    pub fn new(num_cells: usize) -> Self {
        unsafe {
            NUM_CELLS = num_cells;
        }
        
        let mut adj_lists_head = Vec::with_capacity(num_cells);
        for _ in 0..num_cells {
            adj_lists_head.push(None);
        }
        
        Graph {
            adj_lists_head,
            ranges_head: None,
        }
    }
    
    // Add formula to a cell
    pub fn add_formula(&mut self, cell: i32, c1: i32, c2: i32, op_type: i32, formula_array: &mut [Formula]) {
        let mut new_formula = Formula {
            op_type,
            op_info1: -1,
            op_info2: -1,
        };
        
        if op_type == 0 {
            new_formula.op_info1 = c1;
        } else {
            new_formula.op_info1 = c1;
            new_formula.op_info2 = c2;
        }
        
        formula_array[cell as usize] = new_formula;
    }
    
    // Add a cell to an adjacency list
    fn add_cell(cell: i32) -> Box<Cell> {
        Box::new(Cell {
            cell,
            next: None,
        })
    }
    
    // Add a range
    fn add_range(&mut self, start_cell: i32, end_cell: i32, dependent_cell: i32) {
        let new_range = Box::new(Range {
            start_cell,
            end_cell,
            dependent_cell,
            next: self.ranges_head.take(),
        });
        
        self.ranges_head = Some(new_range);
    }
    
    // Add an edge to the linked list
    fn add_edge(&mut self, cell1: i32, head_idx: usize) {
        let head = &mut self.adj_lists_head[head_idx];
        
        // Fast path if list is empty
        if head.is_none() {
            self.adj_lists_head[head_idx] = Some(Self::add_cell(cell1));
            return;
        }
        
        let mut current = head.as_mut().unwrap();
        
        // Fast path if cell1 should be at the beginning
        if current.cell == cell1 {
            return; // Cell already exists, no need to add
        }
        
        // Check if the cell already exists in the list
        while let Some(ref mut next) = current.next {
            if next.cell == cell1 {
                return; // Cell already exists, no need to add
            }
            current = next;
        }
        
        // Add new cell at the end of the list for better cache locality
        current.next = Some(Self::add_cell(cell1));
    }
    
    // Add a range to the graph
    pub fn add_range_to_graph(&mut self, start_cell: i32, end_cell: i32, dependent_cell: i32) {
        self.add_range(start_cell, end_cell, dependent_cell);
    }
    
    // Delete a specific cell from the linked list
    fn delete_cell(&mut self, cell1: i32, head_idx: usize) {
        let head = &mut self.adj_lists_head[head_idx];
        
        if head.is_none() {
            return;
        }
        
        // If the head node itself holds the cell to be deleted
        if head.as_ref().unwrap().cell == cell1 {
            self.adj_lists_head[head_idx] = head.as_mut().unwrap().next.take();
            return;
        }
        
        // Search for the cell to be deleted
        let mut current = head.as_mut().unwrap();
        
        while let Some(ref mut next) = current.next {
            if next.cell == cell1 {
                // Found the cell to delete
                current.next = next.next.take();
                return;
            }
            
            if next.next.is_none() {
                break;
            }
            
            current = current.next.as_mut().unwrap();
        }
    }
    
    // Delete range from the range list
    pub fn delete_range_from_graph(&mut self, dependent_cell: i32) {
        let mut current = &mut self.ranges_head;
        
        while let Some(ref mut range) = *current {
            if range.dependent_cell == dependent_cell {
                // Remove this range
                *current = range.next.take();
            } else {
                current = &mut range.next;
            }
        }
    }
    
    // Delete edge based on formula
    pub fn delete_edge(&mut self, cell: i32, cols: i32, formula_array: &[Formula]) {
        let x = formula_array[cell as usize];
        
        match x.op_type {
            1..=4 => {
                self.delete_cell(cell, x.op_info1 as usize);
            },
            5..=8 => {
                self.delete_cell(cell, x.op_info1 as usize);
                self.delete_cell(cell, x.op_info2 as usize);
            },
            9..=13 => {
                // For range operations, just delete the range entry
                self.delete_range_from_graph(cell);
            },
            14 => {
                // SLEEP operation
                self.delete_cell(cell, x.op_info1 as usize);
            },
            15 => {
                // CONSTANT/CELL operation
                self.delete_cell(cell, x.op_info2 as usize);
            },
            _ => {}
        }
    }
    
    // Add edge based on formula
    pub fn add_edge_formula(&mut self, cell: i32, cols: i32, formula_array: &[Formula]) {
        let x = formula_array[cell as usize];
        
        match x.op_type {
            1..=4 => {
                // For operations 1-4 (single cell operations)
                self.add_edge(cell, x.op_info1 as usize);
            },
            5..=8 => {
                // For operations 5-8 (two cell operations)
                self.add_edge(cell, x.op_info1 as usize);
                self.add_edge(cell, x.op_info2 as usize);
            },
            9..=13 => {
                // For operations 9-13 (range operations)
                let start_cell = x.op_info1;
                let end_cell = x.op_info2;
                self.add_range_to_graph(start_cell, end_cell, cell);
            },
            14 => {
                // For operation 14 (SLEEP)
                // If op_info1 is not the cell itself, it's a reference to another cell
                if x.op_info1 != cell {
                    self.add_edge(cell, x.op_info1 as usize);
                }
                // If op_info1 is the cell itself, it's a constant sleep - no edge needed
            },
            15 => {
                // For operation 15 (CONSTANT/CELL)
                // Add edge from the referenced cell (op_info2)
                self.add_edge(cell, x.op_info2 as usize);
            },
            _ => {}
        }
    }
    
    // Convert linked list to array
    fn get_nodes_from_list(&self, head_idx: usize) -> Vec<i32> {
        let mut nodes = Vec::new();
        let mut current = &self.adj_lists_head[head_idx];
        
        while let Some(ref node) = *current {
            nodes.push(node.cell);
            current = &node.next;
        }
        
        nodes
    }
    
    // DFS for topological sort
    fn dfs(&self, cell: i32, visited: &mut [bool], on_stack: &mut [bool], result: &mut Vec<i32>, has_cycle: &mut bool, cols: i32) {
        // Early exit if cycle already detected
        if *has_cycle {
            return;
        }
        
        // Mark the current node as visited and add to recursion stack
        visited[cell as usize] = true;
        on_stack[cell as usize] = true;
        
        // Visit all adjacent vertices from direct dependencies
        let mut current = &self.adj_lists_head[cell as usize];
        while let Some(ref node) = *current {
            let dependent = node.cell;
            
            // If not visited, then recursively process it
            if !visited[dependent as usize] {
                self.dfs(dependent, visited, on_stack, result, has_cycle, cols);
            }
            // If already in recursion stack, then there's a cycle
            else if on_stack[dependent as usize] {
                *has_cycle = true;
                return;
            }
            
            if *has_cycle {
                return;
            }
            
            current = &node.next;
        }
        
        // Check if this cell is part of any range and add dependencies
        let mut range = &self.ranges_head;
        while let Some(ref r) = *range {
            let start_cell = r.start_cell;
            let end_cell = r.end_cell;
            let dependent = r.dependent_cell;
            
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
            
            let cell_row = cell / cols;
            let cell_col = cell % cols;
            
            // Check if the cell is within the range
            if cell_row >= start_row && cell_row <= end_row &&
               cell_col >= start_col && cell_col <= end_col {
                // If dependent is not visited, recursively process it
                if !visited[dependent as usize] {
                    self.dfs(dependent, visited, on_stack, result, has_cycle, cols);
                }
                // If already in recursion stack, then there's a cycle
                else if on_stack[dependent as usize] {
                    *has_cycle = true;
                    return;
                }
                
                if *has_cycle {
                    return;
                }
            }
            
            range = &r.next;
        }
        
        // Remove from recursion stack and add to result
        on_stack[cell as usize] = false;
        result.push(cell);
    }
    
    // Topological sort from a starting cell
    pub fn topo_sort_from_cell(&self, start_cell: i32, cols: i32) -> Result<Vec<i32>, &'static str> {
        let num_cells = unsafe { NUM_CELLS };
        let mut visited = vec![false; num_cells];
        let mut on_stack = vec![false; num_cells];
        let mut result = Vec::new();
        let mut has_cycle = false;
        
        // Perform DFS starting from the startCell
        self.dfs(start_cell, &mut visited, &mut on_stack, &mut result, &mut has_cycle, cols);
        
        if has_cycle {
            unsafe {
                HAS_CYCLE = true;
            }
            return Err("Circular dependency detected");
        }
        
        // Reverse the result (DFS produces reverse topological order)
        result.reverse();
        
        Ok(result)
    }
    
    // Recalculate cells after a change
    pub fn recalc(&self, cols: i32, arr: &mut [i32], start_cell: i32, formula_array: &[Formula]) -> Result<(), &'static str> {
        let sorted_cells = match self.topo_sort_from_cell(start_cell, cols) {
            Ok(cells) => cells,
            Err(e) => return Err(e),
        };
        
        // Initialize all affected cells to 0
        for &cell in &sorted_cells {
            arr[cell as usize] = 0;
        }
        
        // Process cells in topological order
        for &cell in &sorted_cells {
            let f = formula_array[cell as usize];
            
            match f.op_type {
                0 => {
                    // CELL=CONSTANT
                    arr[cell as usize] = if f.op_info1 == i32::MIN { i32::MIN } else { f.op_info1 };
                },
                1..=4 => {
                    // CELL=CELL+/-/*//CONSTANT
                    let v1 = arr[f.op_info1 as usize];
                    let v2 = f.op_info2;
                    
                    if v1 == i32::MIN {
                        arr[cell as usize] = i32::MIN;
                        continue;
                    }
                    
                    let op = match f.op_type {
                        1 => '+',
                        2 => '-',
                        3 => '*',
                        4 => '/',
                        _ => unreachable!(),
                    };
                    
                    if op == '/' && v2 == 0 {
                        arr[cell as usize] = i32::MIN;
                        continue;
                    }
                    
                    arr[cell as usize] = arithmetic_eval2(v1, v2, op);
                },
                5..=8 => {
                    // CELL=CELL+/-/*//CELL
                    let v1 = arr[f.op_info1 as usize];
                    let v2 = arr[f.op_info2 as usize];
                    
                    if v1 == i32::MIN || v2 == i32::MIN {
                        arr[cell as usize] = i32::MIN;
                        continue;
                    }
                    
                    if f.op_type == 8 && v2 == 0 {
                        arr[cell as usize] = i32::MIN;
                        continue;
                    }
                    
                    let op = match f.op_type {
                        5 => '+',
                        6 => '-',
                        7 => '*',
                        8 => '/',
                        _ => unreachable!(),
                    };
                    
                    arr[cell as usize] = arithmetic_eval2(v1, v2, op);
                },
                9..=13 => {
                    // CELL=MIN/MAX/AVG/SUM/STDEV(RANGE)
                    let start_cell = f.op_info1;
                    let end_cell = f.op_info2;
                    
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
                    let mut values = Vec::new();
                    
                    // First pass: Calculate sum, min, max and check for errors
                    'outer: for row in start_row..=end_row {
                        for col in start_col..=end_col {
                            let idx = row * cols + col;
                            let val = arr[idx as usize];
                            
                            if val == i32::MIN {
                                has_error = true;
                                break 'outer;
                            }
                            
                            sum += val;
                            count += 1;
                            min_val = min(val, min_val);
                            max_val = max(val, max_val);
                            values.push(val);
                        }
                    }
                    
                    if has_error || count == 0 {
                        arr[cell as usize] = i32::MIN;
                        continue;
                    }
                    
                    // Handle different range operations
                    arr[cell as usize] = match f.op_type {
                        9 => min_val,                  // MIN
                        10 => max_val,                 // MAX
                        11 => sum / count,             // AVG
                        12 => sum,                     // SUM
                        13 => std(&values, count as usize), // STDEV
                        _ => unreachable!(),
                    };
                },
                14 => {
                    // CELL=SLEEP(CONSTANT) or CELL=SLEEP(CELL)
                    let mut sleep_value = f.op_info2; // Assuming it's a constant
                    
                    // If op_info1 is not the cell itself, it's a reference to another cell
                    if f.op_info1 != cell {
                        sleep_value = arr[f.op_info1 as usize];
                    }
                    
                    if sleep_value == i32::MIN {
                        arr[cell as usize] = i32::MIN;
                        continue;
                    }
                    
                    if sleep_value > 0 {
                        sleep(Duration::from_secs(sleep_value as u64));
                    }
                    
                    arr[cell as usize] = sleep_value;
                },
                15 => {
                    // CELL=CONSTANT/CELL
                    let v1 = f.op_info1;
                    let v2 = arr[f.op_info2 as usize];
                    
                    if v2 == i32::MIN {
                        arr[cell as usize] = i32::MIN;
                        continue;
                    }
                    
                    let op = '/'; // Division operation
                    
                    if v2 == 0 {
                        arr[cell as usize] = i32::MIN;
                        continue;
                    }
                    
                    arr[cell as usize] = arithmetic_eval2(v1, v2, op);
                },
                _ => {
                    // Handle unknown operation type
                    arr[cell as usize] = i32::MIN;
                },
            }
        }
        
        Ok(())
    }
}

// Implement Drop trait to free resources
impl Drop for Graph {
    fn drop(&mut self) {}
}