use crate::function::Cell;
use crate::function::CellValue;

pub fn arithmetic_eval(v1: Cell, v2: Cell, op: char) -> Cell {
    if !v1.is_valid || !v2.is_valid {
        return Cell::invalid();
    }

    match (&v1.value, &v2.value) {
        (CellValue::Int(i1), CellValue::Int(i2)) => {
            match op {
                '+' => Cell::new_int(i1 + i2),
                '-' => Cell::new_int(i1 - i2),
                '*' => Cell::new_int(i1 * i2),
                '/' => {
                    if *i2 == 0 {
                        Cell::invalid()
                    } else {
                        Cell::new_int(i1 / i2)
                    }
                }
                _ => Cell::invalid(),
            }
        }
        (CellValue::Float(f1), CellValue::Float(f2)) => {
            match op {
                '+' => Cell::new_float(f1 + f2),
                '-' => Cell::new_float(f1 - f2),
                '*' => Cell::new_float(f1 * f2),
                '/' => {
                    if *f2 == 0.0 {
                        Cell::invalid()
                    } else {
                        Cell::new_float(f1 / f2)
                    }
                }
                _ => Cell::invalid(),
            }
        }
        (CellValue::Int(i1), CellValue::Float(f2)) => {
            let f1 = *i1 as f64;
            match op {
                '+' => Cell::new_float(f1 + f2),
                '-' => Cell::new_float(f1 - f2),
                '*' => Cell::new_float(f1 * f2),
                '/' => {
                    if *f2 == 0.0 {
                        Cell::invalid()
                    } else {
                        Cell::new_float(f1 / f2)
                    }
                }
                _ => Cell::invalid(),
            }
        }
        (CellValue::Float(f1), CellValue::Int(i2)) => {
            let f2 = *i2 as f64;
            match op {
                '+' => Cell::new_float(f1 + f2),
                '-' => Cell::new_float(f1 - f2),
                '*' => Cell::new_float(f1 * f2),
                '/' => {
                    if f2 == 0.0 {
                        Cell::invalid()
                    } else {
                        Cell::new_float(f1 / f2)
                    }
                }
                _ => Cell::invalid(),
            }
        }
        _ => Cell::invalid(),
    }
}

pub fn return_optype(op: char) -> i32 {
    match op {
        '+' => 1,
        '-' => 2,
        '*' => 3,
        '/' => 4,
        _ => -1,
    }
}