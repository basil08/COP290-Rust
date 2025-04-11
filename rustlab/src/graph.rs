// use std::cmp::{max, min};
// use std::thread::sleep;
// use std::time::Duration;
// use crate::function::Cell;
// use crate::util::{arithmetic_eval, return_optype};

// #[derive(Clone, Copy)]
// pub struct Formula {
//     pub op_type: i32,
//     pub op_info1: i32,
//     pub op_info2: i32,
// }

// pub struct Cell {
//     pub cell: i32,
//     pub next: Option<Box<Cell>>,
// }

// pub struct Range {
//     pub start_cell: i32,
//     pub end_cell: i32,
//     pub dependent_cell: i32,
//     pub next: Option<Box<Range>>,
// }

// pub struct Graph {
//     pub adj_lists_head: Vec<Option<Box<Cell>>>,
//     pub ranges_head: Option<Box<Range>>,
// }

// impl Graph {
//     pub fn new(num_cells: usize) -> Self {
//         let mut adj_lists_head = Vec::with_capacity(num_cells);
//         for _ in 0..num_cells {
//             adj_lists_head.push(None);
//         }
        
//         Graph {
//             adj_lists_head,
//             ranges_head: None,
//         }
//     }
    
//     pub fn add_formula(&mut self, cell: i32, c1: i32, c2: i32, op_type: i32, formula_array: &mut [Formula]) {
//         let mut new_formula = Formula {
//             op_type,
//             op_info1: -1,
//             op_info2: -1,
//         };
        
//         if op_type == 0 {
//             new_formula.op_info1 = c1;
//         } else {
//             new_formula.op_info1 = c1;
//             new_formula.op_info2 = c2;
//         }
        
//         formula_array[cell as usize] = new_formula;
//     }
    
//     fn add_cell(cell: i32) -> Box<Cell> {
//         Box::new(Cell {
//             cell,
//             next: None,
//         })
//     }
    
//     fn add_range(&mut self, start_cell: i32, end_cell: i32, dependent_cell: i32) {
//         let new_range = Box::new(Range {
//             start_cell,
//             end_cell,
//             dependent_cell,
//             next: self.ranges_head.take(),
//         });
        
//         self.ranges_head = Some(new_range);
//     }
    
//     pub fn add_edge(&mut self, cell1: i32, head_idx: usize) {
//         let head = &mut self.adj_lists_head[head_idx];
        
//         if head.is_none() {
//             self.adj_lists_head[head_idx] = Some(Self::add_cell(cell1));
//             return;
//         }
        
//         let mut current = head.as_mut().unwrap();
        
//         if current.cell == cell1 {
//             return;
//         }
        
//         while let Some(ref mut next) = current.next {
//             if next.cell == cell1 {
//                 return;
//             }
//             current = next;
//         }
        
//         current.next = Some(Self::add_cell(cell1));
//     }
    
//     pub fn add_range_to_graph(&mut self, start_cell: i32, end_cell: i32, dependent_cell: i32) {
//         self.add_range(start_cell, end_cell, dependent_cell);
//     }
    
//     fn delete_cell(&mut self, cell1: i32, head_idx: usize) {
//         let head = &mut self.adj_lists_head[head_idx];
        
//         if head.is_none() {
//             return;
//         }
        
//         if head.as_ref().unwrap().cell == cell1 {
//             self.adj_lists_head[head_idx] = head.as_mut().unwrap().next.take();
//             return;
//         }
        
//         let mut current = head.as_mut().unwrap();
        
//         while let Some(ref mut next) = current.next {
//             if next.cell == cell1 {
//                 current.next = next.next.take();
//                 return;
//             }
            
//             if next.next.is_none() {
//                 break;
//             }
            
//             current = current.next.as_mut().unwrap();
//         }
//     }
    
//     pub fn delete_range_from_graph(&mut self, dependent_cell: i32) {
//         let mut current = &mut self.ranges_head;
        
//         while let Some(ref mut range) = *current {
//             if range.dependent_cell == dependent_cell {
//                 *current = range.next.take();
//             } else {
//                 current = &mut range.next;
//             }
//         }
//     }
    
//     pub fn delete_edge(&mut self, cell: i32, _cols: i32, formula_array: &[Formula]) {
//         let x = formula_array[cell as usize];
        
//         match x.op_type {
//             1..=4 => {
//                 self.delete_cell(cell, x.op_info1 as usize);
//             },
//             5..=8 => {
//                 self.delete_cell(cell, x.op_info1 as usize);
//                 self.delete_cell(cell, x.op_info2 as usize);
//             },
//             9..=13 => {
//                 self.delete_range_from_graph(cell);
//             },
//             14 => {
//                 self.delete_cell(cell, x.op_info1 as usize);
//             },
//             15 => {
//                 self.delete_cell(cell, x.op_info2 as usize);
//             },
//             _ => {}
//         }
//     }
    
//     pub fn add_edge_formula(&mut self, cell: i32, _cols: i32, formula_array: &[Formula]) {
//         let x = formula_array[cell as usize];
        
//         match x.op_type {
//             1..=4 => {
//                 self.add_edge(cell, x.op_info1 as usize);
//             },
//             5..=8 => {
//                 self.add_edge(cell, x.op_info1 as usize);
//                 self.add_edge(cell, x.op_info2 as usize);
//             },
//             9..=13 => {
//                 let start_cell = x.op_info1;
//                 let end_cell = x.op_info2;
//                 self.add_range_to_graph(start_cell, end_cell, cell);
//             },
//             14 => {
//                 if x.op_info1 != cell {
//                     self.add_edge(cell, x.op_info1 as usize);
//                 }
//             },
//             15 => {
//                 self.add_edge(cell, x.op_info2 as usize);
//             },
//             _ => {}
//         }
//     }
    
//     fn dfs(&self, cell: i32, visited: &mut [bool], on_stack: &mut [bool], result: &mut Vec<i32>, has_cycle: &mut bool, cols: i32) {
//         if *has_cycle {
//             return;
//         }
        
//         visited[cell as usize] = true;
//         on_stack[cell as usize] = true;
        
//         let mut current = &self.adj_lists_head[cell as usize];
//         while let Some(ref node) = *current {
//             let dependent = node.cell;
            
//             if !visited[dependent as usize] {
//                 self.dfs(dependent, visited, on_stack, result, has_cycle, cols);
//             } else if on_stack[dependent as usize] {
//                 *has_cycle = true;
//                 return;
//             }
            
//             if *has_cycle {
//                 return;
//             }
            
//             current = &node.next;
//         }
        
//         let mut range = &self.ranges_head;
//         while let Some(ref r) = *range {
//             let start_cell = r.start_cell;
//             let end_cell = r.end_cell;
//             let dependent = r.dependent_cell;
            
//             let start_row = start_cell / cols;
//             let start_col = start_cell % cols;
//             let end_row = end_cell / cols;
//             let end_col = end_cell % cols;
            
//             let (start_row, end_row) = if start_row > end_row {
//                 (end_row, start_row)
//             } else {
//                 (start_row, end_row)
//             };
            
//             let (start_col, end_col) = if start_col > end_col {
//                 (end_col, start_col)
//             } else {
//                 (start_col, end_col)
//             };
            
//             let cell_row = cell / cols;
//             let cell_col = cell % cols;
            
//             if cell_row >= start_row && cell_row <= end_row && cell_col >= start_col && cell_col <= end_col {
//                 if !visited[dependent as usize] {
//                     self.dfs(dependent, visited, on_stack, result, has_cycle, cols);
//                 } else if on_stack[dependent as usize] {
//                     *has_cycle = true;
//                     return;
//                 }
                
//                 if *has_cycle {
//                     return;
//                 }
//             }
            
//             range = &r.next;
//         }
        
//         on_stack[cell as usize] = false;
//         result.push(cell);
//     }
    
//     pub fn topo_sort_from_cell(&self, start_cell: i32, cols: i32, state: &mut State) -> Result<Vec<i32>, &'static str> {
//         let mut visited = vec![false; state.num_cells];
//         let mut on_stack = vec![false; state.num_cells];
//         let mut result = Vec::new();
//         let mut has_cycle = false;
        
//         self.dfs(start_cell, &mut visited, &mut on_stack, &mut result, &mut has_cycle, cols);
        
//         if has_cycle {
//             state.has_cycle = true;
//             return Err("Circular dependency detected");
//         }
        
//         result.reverse();
//         Ok(result)
//     }
    
//     pub fn recalc(&self, cols: i32, arr: &mut [Cell], start_cell: i32, formula_array: &[Formula], state: &mut State) -> Result<(), &'static str> {
//         let sorted_cells = self.topo_sort_from_cell(start_cell, cols, state)?;
        
//         for &cell in &sorted_cells {
//             let f = formula_array[cell as usize];
            
//             match f.op_type {
//                 0 => {
//                     arr[cell as usize] = Cell::new_int(f.op_info1);
//                 },
//                 1..=4 => {
//                     let v1 = arr[f.op_info1 as usize].clone();
//                     let v2 = Cell::new_int(f.op_info2);
//                     if !v1.is_valid {
//                         arr[cell as usize] = Cell::invalid();
//                         continue;
//                     }
//                     let op = match f.op_type {
//                         1 => '+',
//                         2 => '-',
//                         3 => '*',
//                         4 => '/',
//                         _ => unreachable!(),
//                     };
//                     arr[cell as usize] = arithmetic_eval(v1, v2, op);
//                 },
//                 5..=8 => {
//                     let v1 = arr[f.op_info1 as usize].clone();
//                     let v2 = arr[f.op_info2 as usize].clone();
//                     if !v1.is_valid || !v2.is_valid {
//                         arr[cell as usize] = Cell::invalid();
//                         continue;
//                     }
//                     let op = match f.op_type {
//                         5 => '+',
//                         6 => '-',
//                         7 => '*',
//                         8 => '/',
//                         _ => unreachable!(),
//                     };
//                     arr[cell as usize] = arithmetic_eval(v1, v2, op);
//                 },
//                 9..=13 => {
//                     let start_cell = f.op_info1;
//                     let end_cell = f.op_info2;
                    
//                     let start_row = start_cell / cols;
//                     let start_col = start_cell % cols;
//                     let end_row = end_cell / cols;
//                     let end_col = end_cell % cols;
                    
//                     let (start_row, end_row) = if start_row > end_row { (end_row, start_row) } else { (start_row, end_row) };
//                     let (start_col, end_col) = if start_col > end_col { (end_col, start_col) } else { (start_col, end_col) };
                    
//                     let mut sum = 0.0;
//                     let mut count = 0;
//                     let mut min_val = f64::MAX;
//                     let mut max_val = f64::MIN;
//                     let mut has_error = false;
//                     let mut values = Vec::new();
                    
//                     'outer: for row in start_row..=end_row {
//                         for col in start_col..=end_col {
//                             let idx = row * cols + col;
//                             let val = &arr[idx as usize];
//                             if !val.is_valid {
//                                 has_error = true;
//                                 break 'outer;
//                             }
//                             match val.value {
//                                 CellValue::Int(i) => {
//                                     let v = i as f64;
//                                     sum += v;
//                                     count += 1;
//                                     min_val = min_val.min(v);
//                                     max_val = max_val.max(v);
//                                     values.push(v);
//                                 },
//                                 CellValue::Float(f) => {
//                                     sum += f;
//                                     count += 1;
//                                     min_val = min_val.min(f);
//                                     max_val = max_val.max(f);
//                                     values.push(f);
//                                 },
//                                 CellValue::String(_) => {
//                                     has_error = true;
//                                     break 'outer;
//                                 }
//                             }
//                         }
//                     }
                    
//                     if has_error || count == 0 {
//                         arr[cell as usize] = Cell::invalid();
//                         continue;
//                     }
                    
//                     arr[cell as usize] = match f.op_type {
//                         9 => {
//                             if min_val.fract() == 0.0 { Cell::new_int(min_val as i32) } else { Cell::new_float(min_val) }
//                         },
//                         10 => {
//                             if max_val.fract() == 0.0 { Cell::new_int(max_val as i32) } else { Cell::new_float(max_val) }
//                         },
//                         11 => {
//                             let avg = sum / count as f64;
//                             if avg.fract() == 0.0 { Cell::new_int(avg as i32) } else { Cell::new_float(avg) }
//                         },
//                         12 => {
//                             if sum.fract() == 0.0 { Cell::new_int(sum as i32) } else { Cell::new_float(sum) }
//                         },
//                         13 => {
//                             let mean = sum / count as f64;
//                             let variance = values.iter().map(|&x| (x - mean) * (x - mean)).sum::<f64>() / count as f64;
//                             let stdev = variance.sqrt();
//                             if stdev.fract() == 0.0 { Cell::new_int(stdev as i32) } else { Cell::new_float(stdev) }
//                         },
//                         _ => unreachable!(),
//                     };
//                 },
//                 14 => {
//                     let mut sleep_value = arr[f.op_info1 as usize].clone();
//                     if f.op_info1 == cell {
//                         sleep_value = Cell::new_int(f.op_info2);
//                     }
//                     if !sleep_value.is_valid {
//                         arr[cell as usize] = Cell::invalid();
//                         continue;
//                     }
//                     if let CellValue::Int(val) = sleep_value.value {
//                         if val > 0 {
//                             sleep(Duration::from_secs(val as u64));
//                         }
//                     }
//                     arr[cell as usize] = sleep_value;
//                 },
//                 15 => {
//                     let v1 = Cell::new_int(f.op_info1);
//                     let v2 = arr[f.op_info2 as usize].clone();
//                     if !v2.is_valid {
//                         arr[cell as usize] = Cell::invalid();
//                         continue;
//                     }
//                     arr[cell as usize] = arithmetic_eval(v1, v2, '/');
//                 },
//                 _ => {
//                     arr[cell as usize] = Cell::invalid();
//                 },
//             }
//         }
        
//         Ok(())
//     }
// }

// impl Drop for Graph {
//     fn drop(&mut self) {}
// }
use std::cmp::min;
use std::thread::sleep;
use std::time::Duration;
use crate::function::Cell;
use crate::util::arithmetic_eval;

use crate::function::CellValue;

#[derive(Clone, Copy)]
pub struct Formula {
    pub op_type: i32,
    pub op_info1: i32,
    pub op_info2: i32,
}

pub struct GraphNode {
    pub cell: i32,
    pub next: Option<Box<GraphNode>>,
}

pub struct Range {
    pub start_cell: i32,
    pub end_cell: i32,
    pub dependent_cell: i32,
    pub next: Option<Box<Range>>,
}

pub struct Graph {
    pub adj_lists_head: Vec<Option<Box<GraphNode>>>,
    pub ranges_head: Option<Box<Range>>,
}

impl Graph {
    pub fn new(num_cells: usize) -> Self {
        let mut adj_lists_head = Vec::with_capacity(num_cells);
        for _ in 0..num_cells {
            adj_lists_head.push(None);
        }
        Graph {
            adj_lists_head,
            ranges_head: None,
        }
    }

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

    fn add_node(cell: i32) -> Box<GraphNode> {
        Box::new(GraphNode {
            cell,
            next: None,
        })
    }

    fn add_range(&mut self, start_cell: i32, end_cell: i32, dependent_cell: i32) -> Option<Box<Range>> {
        Some(Box::new(Range {
            start_cell,
            end_cell,
            dependent_cell,
            next: self.ranges_head.take(),
        }))
    }

    pub fn add_edge(&mut self, cell1: i32, head_idx: usize) {
        let head = &mut self.adj_lists_head[head_idx];
        if head.is_none() {
            self.adj_lists_head[head_idx] = Some(Self::add_node(cell1));
            return;
        }
        let mut current = head.as_mut().unwrap();
        if current.cell == cell1 {
            return;
        }
        while let Some(ref mut next) = current.next {
            if next.cell == cell1 {
                return;
            }
            current = next;
        }
        current.next = Some(Self::add_node(cell1));
    }

    pub fn add_range_to_graph(&mut self, start_cell: i32, end_cell: i32, dependent_cell: i32) {
        if let Some(new_range) = self.add_range(start_cell, end_cell, dependent_cell) {
            self.ranges_head = Some(new_range);
        }
    }

    fn delete_node(&mut self, cell1: i32, head_idx: usize) {
        let head = &mut self.adj_lists_head[head_idx];
        if head.is_none() {
            return;
        }
        if head.as_ref().unwrap().cell == cell1 {
            self.adj_lists_head[head_idx] = head.as_mut().unwrap().next.take();
            return;
        }
        let mut current = head.as_mut().unwrap();
        while let Some(ref mut next) = current.next {
            if next.cell == cell1 {
                current.next = next.next.take();
                return;
            }
            if next.next.is_none() {
                break;
            }
            current = current.next.as_mut().unwrap();
        }
    }

    
//     void DeleteRangeFromGraph(Graph *graph, int dependentCell)
// {
//     Range *current = graph->ranges_head;
//     Range *prev = NULL;

//     while (current != NULL)
//     {
//         if (current->dependentCell == dependentCell)
//         {
//             // Remove this range
//             if (prev == NULL)
//             {
//                 // It's the head node
//                 graph->ranges_head = current->next;
//                 free(current);
//                 current = graph->ranges_head;
//             }
//             else
//             {
//                 // Middle or end node
//                 prev->next = current->next;
//                 free(current);
//                 current = prev->next;
//             }
//         }
//         else
//         {
//             prev = current;
//             current = current->next;
//         }
//     }
// }

pub fn delete_range_from_graph(&mut self, dependent_cell: i32) {
    let mut current = &mut self.ranges_head;

    while current.is_some() {
        let should_remove = current.as_ref().unwrap().dependent_cell == dependent_cell;
    
        if should_remove {
            let next = current.as_mut().unwrap().next.take();
            *current = next;
            break;
        } else {
            current = &mut current.as_mut().unwrap().next;
        }
    }
    
}


    pub fn delete_edge(&mut self, cell: i32, _cols: i32, formula_array: &[Formula]) {
        let x = formula_array[cell as usize];
        match x.op_type {
            1..=4 => self.delete_node(cell, x.op_info1 as usize),
            5..=8 => {
                self.delete_node(cell, x.op_info1 as usize);
                self.delete_node(cell, x.op_info2 as usize);
            }
            9..=13 => self.delete_range_from_graph(cell),
            14 => self.delete_node(cell, x.op_info1 as usize),
            15 => self.delete_node(cell, x.op_info2 as usize),
            _ => {}
        }
    }

    pub fn add_edge_formula(&mut self, cell: i32, _cols: i32, formula_array: &[Formula]) {
        let x = formula_array[cell as usize];
        match x.op_type {
            1..=4 => self.add_edge(cell, x.op_info1 as usize),
            5..=8 => {
                self.add_edge(cell, x.op_info1 as usize);
                self.add_edge(cell, x.op_info2 as usize);
            }
            9..=13 => {
                let start_cell = x.op_info1;
                let end_cell = x.op_info2;
                self.add_range_to_graph(start_cell, end_cell, cell);
            }
            14 => {
                if x.op_info1 != cell {
                    self.add_edge(cell, x.op_info1 as usize);
                }
            }
            15 => self.add_edge(cell, x.op_info2 as usize),
            _ => {}
        }
    }

    fn dfs(&self, cell: i32, visited: &mut [bool], on_stack: &mut [bool], result: &mut Vec<i32>, has_cycle: &mut bool, cols: i32) {
        if *has_cycle {
            return;
        }
        visited[cell as usize] = true;
        on_stack[cell as usize] = true;
        let mut current = &self.adj_lists_head[cell as usize];
        while let Some(ref node) = current.as_ref() {
            let dependent = node.cell;
            if !visited[dependent as usize] {
                self.dfs(dependent, visited, on_stack, result, has_cycle, cols);
            } else if on_stack[dependent as usize] {
                *has_cycle = true;
                return;
            }
            if *has_cycle {
                return;
            }
            current = &node.next;
        }
        let mut range = &self.ranges_head;
        while let Some(ref r) = range.as_ref() {
            let start_cell = r.start_cell;
            let end_cell = r.end_cell;
            let dependent = r.dependent_cell;
            let start_row = start_cell / cols;
            let start_col = start_cell % cols;
            let end_row = end_cell / cols;
            let end_col = end_cell % cols;
            let (start_row, end_row) = if start_row > end_row { (end_row, start_row) } else { (start_row, end_row) };
            let (start_col, end_col) = if start_col > end_col { (end_col, start_col) } else { (start_col, end_col) };
            let cell_row = cell / cols;
            let cell_col = cell % cols;
            if cell_row >= start_row && cell_row <= end_row && cell_col >= start_col && cell_col <= end_col {
                if !visited[dependent as usize] {
                    self.dfs(dependent, visited, on_stack, result, has_cycle, cols);
                } else if on_stack[dependent as usize] {
                    *has_cycle = true;
                    return;
                }
                if *has_cycle {
                    return;
                }
            }
            range = &r.next;
        }
        on_stack[cell as usize] = false;
        result.push(cell);
    }

    pub fn topo_sort_from_cell(&self, start_cell: i32, cols: i32, state: &mut State) -> Result<Vec<i32>, &'static str> {
        let mut visited = vec![false; state.num_cells];
        let mut on_stack = vec![false; state.num_cells];
        let mut result = Vec::new();
        let mut has_cycle = false;
        self.dfs(start_cell, &mut visited, &mut on_stack, &mut result, &mut has_cycle, cols);
        if has_cycle {
            state.has_cycle = true;
            return Err("Circular dependency detected");
        }
        result.reverse();
        Ok(result)
    }

    pub fn recalc(&self, cols: i32, arr: &mut [Cell], start_cell: i32, formula_array: &[Formula], state: &mut State) -> Result<(), &'static str> {
        let sorted_cells = self.topo_sort_from_cell(start_cell, cols, state)?;
        for &cell in &sorted_cells {
            let f = formula_array[cell as usize];
            match f.op_type {
                0 => arr[cell as usize] = Cell::new_int(f.op_info1),
                1..=4 => {
                    let v1 = arr[f.op_info1 as usize].clone();
                    let v2 = Cell::new_int(f.op_info2);
                    if !v1.is_valid {
                        arr[cell as usize] = Cell::invalid();
                        continue;
                    }
                    let op = match f.op_type {
                        1 => '+',
                        2 => '-',
                        3 => '*',
                        4 => '/',
                        _ => unreachable!(),
                    };
                    arr[cell as usize] = arithmetic_eval(v1, v2, op);
                }
                5..=8 => {
                    let v1 = arr[f.op_info1 as usize].clone();
                    let v2 = arr[f.op_info2 as usize].clone();
                    if !v1.is_valid || !v2.is_valid {
                        arr[cell as usize] = Cell::invalid();
                        continue;
                    }
                    let op = match f.op_type {
                        5 => '+',
                        6 => '-',
                        7 => '*',
                        8 => '/',
                        _ => unreachable!(),
                    };
                    arr[cell as usize] = arithmetic_eval(v1, v2, op);
                }
                9..=13 => {
                    let start_cell = f.op_info1;
                    let end_cell = f.op_info2;
                    let start_row = start_cell / cols;
                    let start_col = start_cell % cols;
                    let end_row = end_cell / cols;
                    let end_col = end_cell % cols;
                    let (start_row, end_row) = if start_row > end_row { (end_row, start_row) } else { (start_row, end_row) };
                    let (start_col, end_col) = if start_col > end_col { (end_col, start_col) } else { (start_col, end_col) };
                    let mut sum = 0.0;
                    let mut count = 0;
                    let mut min_val = f64::MAX;
                    let mut max_val = f64::MIN;
                    let mut has_error = false;
                    let mut values = Vec::new();
                    'outer: for row in start_row..=end_row {
                        for col in start_col..=end_col {
                            let idx = row * cols + col;
                            let val = &arr[idx as usize];
                            if !val.is_valid {
                                has_error = true;
                                break 'outer;
                            }
                            match &val.value {
                                CellValue::Int(i) => {
                                    let v = *i as f64;
                                    sum += v;
                                    count += 1;
                                    min_val = min_val.min(v);
                                    max_val = max_val.max(v);
                                    values.push(v);
                                }
                                CellValue::Float(f) => {
                                    sum += *f;
                                    count += 1;
                                    min_val = min_val.min(*f);
                                    max_val = max_val.max(*f);
                                    values.push(*f);
                                }
                                CellValue::String(_) => {
                                    has_error = true;
                                    break 'outer;
                                }
                            }
                        }
                    }
                    if has_error || count == 0 {
                        arr[cell as usize] = Cell::invalid();
                        continue;
                    }
                    arr[cell as usize] = match f.op_type {
                        9 => if min_val.fract() == 0.0 { Cell::new_int(min_val as i32) } else { Cell::new_float(min_val) },
                        10 => if max_val.fract() == 0.0 { Cell::new_int(max_val as i32) } else { Cell::new_float(max_val) },
                        11 => {
                            let avg = sum / count as f64;
                            if avg.fract() == 0.0 { Cell::new_int(avg as i32) } else { Cell::new_float(avg) }
                        }
                        12 => if sum.fract() == 0.0 { Cell::new_int(sum as i32) } else { Cell::new_float(sum) },
                        13 => {
                            let mean = sum / count as f64;
                            let variance = values.iter().map(|&x| (x - mean) * (x - mean)).sum::<f64>() / count as f64;
                            let stdev = variance.sqrt();
                            if stdev.fract() == 0.0 { Cell::new_int(stdev as i32) } else { Cell::new_float(stdev) }
                        }
                        _ => unreachable!(),
                    };
                }
                14 => {
                    let mut sleep_value = arr[f.op_info1 as usize].clone();
                    if f.op_info1 == cell {
                        sleep_value = Cell::new_int(f.op_info2);
                    }
                    if !sleep_value.is_valid {
                        arr[cell as usize] = Cell::invalid();
                        continue;
                    }
                    if let CellValue::Int(val) = sleep_value.value {
                        if val > 0 {
                            sleep(Duration::from_secs(val as u64));
                        }
                    }
                    arr[cell as usize] = sleep_value;
                }
                15 => {
                    let v1 = Cell::new_int(f.op_info1);
                    let v2 = arr[f.op_info2 as usize].clone();
                    if !v2.is_valid {
                        arr[cell as usize] = Cell::invalid();
                        continue;
                    }
                    arr[cell as usize] = arithmetic_eval(v1, v2, '/');
                }
                _ => arr[cell as usize] = Cell::invalid(),
            }
        }
        Ok(())
    }
}

impl Drop for Graph {
    fn drop(&mut self) {}
}


#[derive(Default, Clone)]
pub struct State {
    pub old_value: Cell,
    pub old_op_type: i32,
    pub old_op_info1: i32,
    pub old_op_info2: i32,
    pub has_cycle: bool,
    pub num_cells: usize,
}

impl State {
    pub fn new() -> Self {
        State {
            old_value: Cell::invalid(),
            old_op_type: 0,
            old_op_info1: 0,
            old_op_info2: 0,
            has_cycle: false,
            num_cells: 0,
        }
    }
}
