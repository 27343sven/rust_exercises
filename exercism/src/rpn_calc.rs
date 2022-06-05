#![allow(unused)]

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    // let test: Result<Vec<i32>> = inputs.iter().try_fold(vec![], | mut acc, input|)

    let mut stack: Result<Vec<i32>, &str> =
        inputs
            .iter()
            .try_fold(vec![], |mut acc, input| match input {
                CalculatorInput::Value(x) => {
                    acc.push(*x);
                    Ok(acc)
                }
                _ => {
                    let numbers = match (acc.pop(), acc.pop()) {
                        (Some(n2), Some(n1)) => Some([n1, n2]),
                        _ => None,
                    };

                    if let Some([x1, x2]) = numbers {
                        match input {
                            CalculatorInput::Add => acc.push(x1 + x2),
                            CalculatorInput::Subtract => acc.push(x1 - x2),
                            CalculatorInput::Multiply => acc.push(x1 * x2),
                            CalculatorInput::Divide => acc.push(i32::from(x1 / x2)),
                            _ => {}
                        }
                        Ok(acc)
                    } else {
                        Err("To few arguments")
                    }
                }
            });
    
    if let Ok(result) = stack {
        match result.get(0) {
            Some(n) if result.len() == 1 => {
                Some(*n)
            }
            _ => None,
        }
    } else {
        None
    }
}

#[test]
fn simple_operations() {
    let operations: [CalculatorInput; 7] = [
        CalculatorInput::Value(4),
        CalculatorInput::Value(8),
        CalculatorInput::Add,
        CalculatorInput::Value(7),
        CalculatorInput::Value(5),
        CalculatorInput::Subtract,
        CalculatorInput::Divide,
    ];
    assert_eq!(6, evaluate(&operations).unwrap_or(0));
}
