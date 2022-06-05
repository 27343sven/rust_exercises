#[allow(unused)]
use std::collections::HashMap as Test;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    definitions: Test<String, Vec<Operation>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Add,
    Sub,
    Div,
    Mul,
    Dup,
    Drop,
    Swap,
    Over,
    DefStart,
    DefEnd,
    Value(i32),
    Symbol(String),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack: Vec::new(),
            definitions: Test::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut operations = Forth::parse_string(input);
        while let Some(op) = operations.pop() {
            self.eval_op(&mut operations, op)?;
        }
        println!("[*] done! {:?}", self.stack);
        Ok(())
    }

    fn define_symbol(&mut self, name: String) -> Option<Vec<Operation>> {
        let result = if let Some(ops) = self.definitions.get(&name) {
            Some(ops.to_vec())
        } else if let Some(op) = Forth::match_op(name.clone()) {
            Some(vec![op])
        } else {
            None
        };
        println!("[|]\t '{}' => {:?}", name, result);
        result
    }

    fn eval_op(&mut self, operations: &mut Vec<Operation>, op: Operation) -> Result {
        println!("[*] {:?} {:?}", self.stack, op);
        match op {
            Operation::Value(n) => {
                self.stack.push(n);
                Ok(())
            }
            Operation::Symbol(sym) => {
                if let Some(mut ops) = self.define_symbol(sym) {
                    operations.append(&mut ops);
                    Ok(())
                } else {
                    Err(Error::UnknownWord)
                }
            }
            Operation::Add => {
                let n2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let n1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(n1 + n2);
                Ok(())
            }
            Operation::Sub => {
                let n2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let n1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(n1 - n2);
                Ok(())
            }
            Operation::Mul => {
                let n2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let n1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(n1 * n2);
                Ok(())
            }
            Operation::Div => {
                let n2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let n1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                if n2 == 0 {
                    return Err(Error::DivisionByZero);
                }
                self.stack.push(n1 / n2);
                Ok(())
            }
            Operation::Dup => {
                let n = self.stack.last().copied().ok_or(Error::StackUnderflow)?;
                self.stack.push(n);
                Ok(())
            }
            Operation::Drop => {
                self.stack.pop().ok_or(Error::StackUnderflow)?;
                Ok(())
            }
            Operation::Swap => {
                let n2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let n1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(n2);
                self.stack.push(n1);
                Ok(())
            }
            Operation::Over => {
                let n = self
                    .stack
                    .get(self.stack.len().wrapping_sub(2))
                    .copied()
                    .ok_or(Error::StackUnderflow)?;
                self.stack.push(n);
                Ok(())
            }
            Operation::DefStart => {
                let name: String;
                if let Some(Operation::Symbol(n)) = operations.pop() {
                    if n.chars().all(|c| !c.is_numeric()) {
                        name = n.to_lowercase();
                    } else {
                        return Err(Error::InvalidWord);
                    }
                } else {
                    return Err(Error::InvalidWord);
                }

                let mut new_ops: Vec<Operation> = Vec::new();
                let mut has_end = false;

                while let Some(op) = operations.pop() {
                    match op {
                        Operation::Symbol(name) => match self.define_symbol(name) {
                            Some(mut ops) => {
                                if ops.get(0).unwrap_or(&Operation::Add) == &Operation::DefEnd {
                                    has_end = true;
                                    break;
                                }
                                new_ops.append(&mut ops);
                            }
                            _ => return Err(Error::InvalidWord),
                        },
                        _ => new_ops.push(op),
                    }
                }
                if !has_end {
                    return Err(Error::InvalidWord);
                }
                new_ops.reverse();
                self.definitions.insert(name, new_ops);
                Ok(())
            }
            _ => Err(Error::InvalidWord),
        }
    }

    fn match_op(op: String) -> Option<Operation> {
        match op.as_str() {
            "+" => Some(Operation::Add),
            "-" => Some(Operation::Sub),
            "*" => Some(Operation::Mul),
            "/" => Some(Operation::Div),
            ":" => Some(Operation::DefStart),
            ";" => Some(Operation::DefEnd),
            "dup" => Some(Operation::Dup),
            "drop" => Some(Operation::Drop),
            "swap" => Some(Operation::Swap),
            "over" => Some(Operation::Over),
            _ => None,
        }
    }

    fn parse_string(input: &str) -> Vec<Operation> {
        input
            .to_lowercase()
            .split(' ')
            .map(|op| {
                let num = op.parse::<i32>();
                if let Ok(n) = num {
                    Operation::Value(n)
                } else {
                    Operation::Symbol(op.to_string())
                }
            })
            .rev()
            .collect()
        // let mut a = vec![
        //     Operation::Value(3),
        //     Operation::Value(5),
        //     Operation::Add,
        // ];
        // a.reverse();
        // a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_input_no_stack() {
        assert_eq!(Vec::<Value>::new(), Forth::new().stack());
    }

    #[test]
    fn numbers_just_get_pushed_onto_the_stack() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 3 4 5").is_ok());

        assert_eq!(vec![1, 2, 3, 4, 5], f.stack());
    }

    #[test]

    fn can_add_two_numbers() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 +").is_ok());

        assert_eq!(vec![3], f.stack());
    }

    #[test]

    fn addition_error() {
        let mut f = Forth::new();

        assert_eq!(Err(Error::StackUnderflow), f.eval("1 +"));

        assert_eq!(Err(Error::StackUnderflow), f.eval("+"));
    }

    #[test]

    fn can_subtract_two_numbers() {
        let mut f = Forth::new();

        assert!(f.eval("3 4 -").is_ok());

        assert_eq!(vec![-1], f.stack());
    }

    #[test]

    fn subtraction_error() {
        let mut f = Forth::new();

        assert_eq!(Err(Error::StackUnderflow), f.eval("1 -"));

        assert_eq!(Err(Error::StackUnderflow), f.eval("-"));
    }

    #[test]

    fn can_multiply_two_numbers() {
        let mut f = Forth::new();

        assert!(f.eval("2 4 *").is_ok());

        assert_eq!(vec![8], f.stack());
    }

    #[test]

    fn multiplication_error() {
        let mut f = Forth::new();

        assert_eq!(Err(Error::StackUnderflow), f.eval("1 *"));

        assert_eq!(Err(Error::StackUnderflow), f.eval("*"));
    }

    #[test]

    fn can_divide_two_numbers() {
        let mut f = Forth::new();

        assert!(f.eval("12 3 /").is_ok());

        assert_eq!(vec![4], f.stack());
    }

    #[test]

    fn performs_integer_division() {
        let mut f = Forth::new();

        assert!(f.eval("8 3 /").is_ok());

        assert_eq!(vec![2], f.stack());
    }

    #[test]

    fn division_error() {
        let mut f = Forth::new();

        assert_eq!(Err(Error::StackUnderflow), f.eval("1 /"));

        assert_eq!(Err(Error::StackUnderflow), f.eval("/"));
    }

    #[test]

    fn errors_if_dividing_by_zero() {
        let mut f = Forth::new();

        assert_eq!(Err(Error::DivisionByZero), f.eval("4 0 /"));
    }

    #[test]

    fn addition_and_subtraction() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 + 4 -").is_ok());

        assert_eq!(vec![-1], f.stack());
    }

    #[test]

    fn multiplication_and_division() {
        let mut f = Forth::new();

        assert!(f.eval("2 4 * 3 /").is_ok());

        assert_eq!(vec![2], f.stack());
    }

    #[test]

    fn dup() {
        let mut f = Forth::new();

        assert!(f.eval("1 dup").is_ok());

        assert_eq!(vec![1, 1], f.stack());
    }

    #[test]

    fn dup_top_value_only() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 dup").is_ok());

        assert_eq!(vec![1, 2, 2], f.stack());
    }

    #[test]

    fn dup_case_insensitive() {
        let mut f = Forth::new();

        assert!(f.eval("1 DUP Dup dup").is_ok());

        assert_eq!(vec![1, 1, 1, 1], f.stack());
    }

    #[test]

    fn dup_error() {
        let mut f = Forth::new();

        assert_eq!(Err(Error::StackUnderflow), f.eval("dup"));
    }

    #[test]

    fn drop() {
        let mut f = Forth::new();

        assert!(f.eval("1 drop").is_ok());

        assert_eq!(Vec::<Value>::new(), f.stack());
    }

    #[test]

    fn drop_with_two() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 drop").is_ok());

        assert_eq!(vec![1], f.stack());
    }

    #[test]

    fn drop_case_insensitive() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 3 4 DROP Drop drop").is_ok());

        assert_eq!(vec![1], f.stack());
    }

    #[test]

    fn drop_error() {
        let mut f = Forth::new();

        assert_eq!(Err(Error::StackUnderflow), f.eval("drop"));
    }

    #[test]

    fn swap() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 swap").is_ok());

        assert_eq!(vec![2, 1], f.stack());
    }

    #[test]

    fn swap_with_three() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 3 swap").is_ok());

        assert_eq!(vec![1, 3, 2], f.stack());
    }

    #[test]

    fn swap_case_insensitive() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 SWAP 3 Swap 4 swap").is_ok());

        assert_eq!(vec![2, 3, 4, 1], f.stack());
    }

    #[test]

    fn swap_error() {
        let mut f = Forth::new();

        assert_eq!(Err(Error::StackUnderflow), f.eval("1 swap"));

        assert_eq!(Err(Error::StackUnderflow), f.eval("swap"));
    }

    #[test]

    fn over() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 over").is_ok());

        assert_eq!(vec![1, 2, 1], f.stack());
    }

    #[test]

    fn over_with_three() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 3 over").is_ok());

        assert_eq!(vec![1, 2, 3, 2], f.stack());
    }

    #[test]

    fn over_case_insensitive() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 OVER Over over").is_ok());

        assert_eq!(vec![1, 2, 1, 2, 1], f.stack());
    }

    #[test]

    fn over_error() {
        let mut f = Forth::new();

        assert_eq!(Err(Error::StackUnderflow), f.eval("1 over"));

        assert_eq!(Err(Error::StackUnderflow), f.eval("over"));
    }

    // User-defined words

    #[test]

    fn can_consist_of_built_in_words() {
        let mut f = Forth::new();

        assert!(f.eval(": dup-twice dup dup ;").is_ok());

        assert!(f.eval("1 dup-twice").is_ok());

        assert_eq!(vec![1, 1, 1], f.stack());
    }

    #[test]

    fn execute_in_the_right_order() {
        let mut f = Forth::new();

        assert!(f.eval(": countup 1 2 3 ;").is_ok());

        assert!(f.eval("countup").is_ok());

        assert_eq!(vec![1, 2, 3], f.stack());
    }

    #[test]

    fn redefining_an_existing_word() {
        let mut f = Forth::new();

        assert!(f.eval(": foo dup ;").is_ok());

        assert!(f.eval(": foo dup dup ;").is_ok());

        assert!(f.eval("1 foo").is_ok());

        assert_eq!(vec![1, 1, 1], f.stack());
    }

    #[test]

    fn redefining_an_existing_built_in_word() {
        let mut f = Forth::new();

        assert!(f.eval(": swap dup ;").is_ok());

        assert!(f.eval("1 swap").is_ok());

        assert_eq!(vec![1, 1], f.stack());
    }

    #[test]

    fn user_defined_words_are_case_insensitive() {
        let mut f = Forth::new();

        assert!(f.eval(": foo dup ;").is_ok());

        assert!(f.eval("1 FOO Foo foo").is_ok());

        assert_eq!(vec![1, 1, 1, 1], f.stack());
    }

    #[test]

    fn definitions_are_case_insensitive() {
        let mut f = Forth::new();

        assert!(f.eval(": SWAP DUP Dup dup ;").is_ok());

        assert!(f.eval("1 swap").is_ok());

        assert_eq!(vec![1, 1, 1, 1], f.stack());
    }

    #[test]

    fn redefining_a_built_in_operator() {
        let mut f = Forth::new();

        assert!(f.eval(": + * ;").is_ok());

        assert!(f.eval("3 4 +").is_ok());

        assert_eq!(vec![12], f.stack());
    }

    #[test]

    fn can_use_different_words_with_the_same_name() {
        let mut f = Forth::new();

        assert!(f.eval(": foo 5 ;").is_ok());

        assert!(f.eval(": bar foo ;").is_ok());

        assert!(f.eval(": foo 6 ;").is_ok());

        assert!(f.eval("bar foo").is_ok());

        assert_eq!(vec![5, 6], f.stack());
    }

    #[test]

    fn can_define_word_that_uses_word_with_the_same_name() {
        let mut f = Forth::new();

        assert!(f.eval(": foo 10 ;").is_ok());

        assert!(f.eval(": foo foo 1 + ;").is_ok());

        assert!(f.eval("foo").is_ok());

        assert_eq!(vec![11], f.stack());
    }

    #[test]

    fn defining_a_number() {
        let mut f = Forth::new();

        assert_eq!(Err(Error::InvalidWord), f.eval(": 1 2 ;"));
    }

    #[test]

    fn malformed_word_definition() {
        let mut f = Forth::new();

        assert_eq!(Err(Error::InvalidWord), f.eval(":"));

        assert_eq!(Err(Error::InvalidWord), f.eval(": foo"));

        assert_eq!(Err(Error::InvalidWord), f.eval(": foo 1"));
    }

    #[test]

    fn calling_non_existing_word() {
        let mut f = Forth::new();

        assert_eq!(Err(Error::UnknownWord), f.eval("1 foo"));
    }

    #[test]

    fn multiple_definitions() {
        let mut f = Forth::new();

        assert!(f.eval(": one 1 ; : two 2 ; one two +").is_ok());

        assert_eq!(vec![3], f.stack());
    }

    #[test]

    fn definitions_after_ops() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 + : addone 1 + ; addone").is_ok());
        assert_eq!(vec![4], f.stack());
    }

    #[test]

    fn redefine_an_existing_word_with_another_existing_word() {
        let mut f = Forth::new();

        assert!(f.eval(": foo 5 ;").is_ok());

        assert!(f.eval(": bar foo ;").is_ok());

        assert!(f.eval(": foo 6 ;").is_ok());

        assert!(f.eval(": bar foo ;").is_ok());

        assert!(f.eval("bar foo").is_ok());

        assert_eq!(vec![6, 6], f.stack());
    }

    #[test]
    fn fibonacci() {
        let mut f = Forth::new();

        assert!(f.eval(": fib over over + ;").is_ok());

        assert!(f.eval("1 1 fib fib fib fib").is_ok());

        assert_eq!(vec![1, 1, 2, 3, 5, 8], f.stack());
    }
}
