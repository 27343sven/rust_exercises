
use std::error::Error;
use std::io;
/**
 * all this code is wrong because it doesn't cache results after calculating a cell value
 */
macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Copy, Clone)]
enum CellType {
    Ref(usize),
    Val(i32),
}

#[derive(Copy, Clone)]
enum Operation {
    Val,
    Add,
    Sub,
    Mult,
}

#[derive(Copy, Clone)]
struct CellValue {
    kind: CellType,
    cache: Option<i32>,
}

impl CellValue {
    fn new(kind: CellType) -> CellValue {
        CellValue { kind, cache: None }
    }

    fn value(&self) -> CellType {
        match self.cache {
            Some(val) => CellType::Val(val),
            None => self.kind,
        }
    }
}

struct Sheet {
    cells: Vec<Cell>,
}

impl Sheet {
    fn new() -> Sheet {
        Sheet { cells: vec![] }
    }

    fn push(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    fn cell_value(&mut self, cell: &mut Option<CellValue>) -> i32 {
        match cell {
            Some(cell) => match cell.value() {
                CellType::Ref(reference) => {
                    let val = self.calculate_cell(reference);
                    cell.cache = Some(val);
                    val
                }
                CellType::Val(val) => val,
            },
            None => 0,
        }
    }

    fn calculate_cell(&mut self, index: usize) -> i32 {
        let mut cell = self.cells[index];
        let arg1 = self.cell_value(&mut cell.arg1);
        let arg2 = self.cell_value(&mut cell.arg2);

        match self.cells[index].operation {
            Operation::Val => arg1,
            Operation::Add => arg1 + arg2,
            Operation::Sub => arg1 - arg2,
            Operation::Mult => arg1 * arg2,
        }
    }
}

#[derive(Copy, Clone)]
struct Cell {
    operation: Operation,
    arg1: Option<CellValue>,
    arg2: Option<CellValue>,
}

impl Cell {}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let mut sheet = Sheet::new();

    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let operation = parse_operation(inputs[0].trim()).unwrap();
        let arg1 = parse_arg(inputs[1].trim()).unwrap();
        let arg2 = parse_arg(inputs[2].trim()).unwrap();

        sheet.push(Cell {
            operation,
            arg1,
            arg2,
        })
    }
    for i in 0..n as usize {
        // Write an answer using println!("message...");
        // To debug: eprintln!("Debug message...");
        println!("{}", sheet.calculate_cell(i));
    }
}

fn parse_operation(operation: &str) -> Result<Operation, String> {
    match operation {
        "VALUE" => Ok(Operation::Val),
        "ADD" => Ok(Operation::Add),
        "SUB" => Ok(Operation::Sub),
        "MULT" => Ok(Operation::Mult),
        _ => Err(format!("Couldn't parse operation '{}'.", operation)),
    }
}

fn parse_arg(arg: &str) -> Result<Option<CellValue>, Box<dyn Error>> {
    match arg {
        "_" => Ok(None),
        _ => {
            match arg
                .chars()
                .nth(0)
                .expect("Couldn't parse argument because it was empty.")
            {
                '$' => {
                    let reference = arg[1..].parse::<usize>()?;
                    Ok(Some(CellValue::new(CellType::Ref(reference))))
                }
                _ => {
                    let value = arg.parse::<i32>()?;
                    Ok(Some(CellValue::new(CellType::Val(value))))
                }
            }
        }
    }
}
