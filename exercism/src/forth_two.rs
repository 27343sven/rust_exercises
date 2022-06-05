#[allow(unused)]
use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    definitions: HashMap<String, Vec<Operation>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionType {
    Definition,
    Loop,
    If,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    Dup,
    Drop,
    Swap,
    Over,
    ExpStart(ExpressionType),
    ExpEnd,
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
            definitions: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut operations = Forth::parse_string(input);
        self.eval_ops(&mut operations)?;

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
        println!("[|]\t => '{}' {:?}", name, result.clone().unwrap_or(vec![]));
        result
    }

    fn eval_ops(&mut self, operations: &mut Vec<Operation>) -> Result {
        while let Some(op) = operations.pop() {
            self.eval_op(operations, op)?;
        }
        Ok(())
    }

    fn eval_op(&mut self, operations: &mut Vec<Operation>, op: Operation) -> Result {
        println!("[*] {:?} {:?}", op, self.stack);
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
            Operation::Mod => {
                let n2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let n1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(n1 % n2);
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
            Operation::ExpStart(ExpressionType::Definition) => {
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
                let new_ops = self.take_op_until(operations, Operation::ExpEnd, true)?;

                self.definitions.insert(name, new_ops);
                Ok(())
            }
            Operation::ExpStart(ExpressionType::Loop) => {
                let mut n_ops = self.take_op_until(
                    operations,
                    Operation::ExpStart(ExpressionType::Definition),
                    true,
                )?;
                self.eval_ops(&mut n_ops)?;
                let n = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let mut loop_ops = self.take_op_until(operations, Operation::ExpEnd, true)?;
                println!("loop {:?}", loop_ops);
                for _ in 0..n {
                    operations.append(&mut loop_ops.clone());
                }
                Ok(())
            }
            _ => Err(Error::InvalidWord),
        }
    }

    fn take_op_until(
        &mut self,
        operations: &mut Vec<Operation>,
        cmp_op: Operation,
        eval_symbols: bool,
    ) -> std::result::Result<Vec<Operation>, Error> {
        let mut new_ops: Vec<Operation> = Vec::new();
        let mut has_end = false;

        while let Some(op) = operations.pop() {
            println!("[|]\t {:?}", op);
            match op {
                Operation::Symbol(name) => match if eval_symbols {
                    self.define_symbol(name.clone())
                } else {
                    Forth::match_op(name.clone()).and_then(|op| Some(vec![op]))
                } {
                    // Some([Operation::Drop]) => {}
                    Some(mut ops) => {
                        let first_op = ops.get(0).unwrap_or(&Operation::Add);
                        if first_op == &cmp_op {
                            has_end = true;
                            break;
                        } else if first_op == &Operation::ExpStart(ExpressionType::Definition) {
                            let mut def_ops =
                                self.take_op_until(operations, Operation::ExpEnd, false)?;
                            ops.append(&mut def_ops);
                        }

                        ops.reverse();

                        if eval_symbols {
                            new_ops.append(&mut ops);
                        } else {
                            new_ops.push(Operation::Symbol(name));
                        }
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
        Ok(new_ops)
    }

    fn match_op(op: String) -> Option<Operation> {
        match op.as_str() {
            "+" => Some(Operation::Add),
            "-" => Some(Operation::Sub),
            "*" => Some(Operation::Mul),
            "/" => Some(Operation::Div),
            "%" => Some(Operation::Mod),
            "dup" => Some(Operation::Dup),
            "drop" => Some(Operation::Drop),
            "swap" => Some(Operation::Swap),
            "over" => Some(Operation::Over),
            ":" => Some(Operation::ExpStart(ExpressionType::Definition)),
            "loop" => Some(Operation::ExpStart(ExpressionType::Loop)),
            ";" => Some(Operation::ExpEnd),
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
    }
}

fn n_to_bottles(n: u32, capitalize: bool) -> String {
    match n {
        0 => if capitalize {"N"} else {"n"}.to_string() + "o more bottles",
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n),
    }
}

fn n_to_action(n: u32) -> String {
    match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    }
    .to_string()
}

pub fn verse(n: u32) -> String {
    let mut verse: String = String::new();
    verse
        .push_str(&(n_to_bottles(n, true) + " of beer on the wall, " + &n_to_bottles(n, false) + " of beer.\n"));
    verse.push_str(&(n_to_action(n) + ", "));
    let new_n = n.checked_sub(1).unwrap_or(99);
    verse.push_str(&(n_to_bottles(new_n, false) + " of beer on the wall.\n"));
    verse
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .step_by(1)
        .fold(String::new(), |mut song, n| {
            song.push_str(&(verse(n) + if n != end { "\n" } else { "" }));
            song
        })
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_song_3_0() {
        assert_eq!(sing(3, 0), "3 bottles of beer on the wall, 3 bottles of beer.\nTake one down and pass it around, 2 bottles of beer on the wall.\n\n2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }

    #[test]
    fn test_verse_1() {
        assert_eq!(verse(1), "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    }
    #[test]
    fn no_input_no_stack() {
        println!("!!! {}", sing(20, 0));
        // assert_eq!(Vec::<Value>::new(), Forth::new().stack());
    }

    #[test]
    fn fibbonacci() {
        let mut f = Forth::new();

        assert!(f.eval(": fib over over + ;").is_ok());
        println!("{:?}", f.eval("1 1 loop 10 : fib ;"));
    }

    #[test]
    fn primes() {
        let mut f = Forth::new();
        // f.eval(": is-div-n ").unwrap();
        assert!(f.eval(": test : set 3 ; ;").is_ok());
        // assert!(f.eval(": set : set 3 ; ;").is_ok());
        // assert!(f.eval("set").is_ok());

        println!("{:?}", f.eval("set"));
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
}
