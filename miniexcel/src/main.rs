use std::error::Error;
use std::io;

mod lib;
use lib::*;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let mut sheet = Sheet::new();

    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let operation = parse_operation(inputs[0].trim()).unwrap();
        let arg1 = parse_arg(inputs[1].trim()).unwrap();
        let arg2 = parse_arg(inputs[2].trim()).unwrap();

        sheet.push(Cell {
            operation,
            args: [arg1, arg2],
        });
    }
    for i in 0..n as usize {
        // Write an answer using println!("message...");
        // To debug: eprintln!("Debug message...");
        println!("{}", sheet.calculate_cell(i));
    }
    // println!("calculated {} times", sheet.get_total());
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
                .expect("Couldn't parse argument.")
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
