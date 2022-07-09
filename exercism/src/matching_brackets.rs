#[allow(dead_code, unused)]
use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut counts: HashMap<char, u32> = HashMap::new();
    let brackets = [['{', '}'], ['[', ']'], ['(', ')']];
    let mut bracket_stack: Vec<char> = Vec::new();

    let test: Option<HashMap<char, u32>> = string.chars().try_fold(counts, |counts, c| {
        brackets.iter().try_fold(counts, |mut acc, [start, end]| {
            if &c == start {
                *acc.entry(*start).or_insert(0) += 1;
                bracket_stack.push(*start);
            } else if &c == end {
                bracket_stack
                    .last()
                    .and_then(|last_start| (start == last_start).then_some(last_start))?;
                bracket_stack.pop();
                let count = acc.get_mut(start)?;
                *count = count.checked_sub(1)?;
            }
            Some(acc)
        })
    });
    test.and_then(|count| count.iter().all(|(_, n)| *n == 0u32).then_some(false))
        .is_some()
}

mod tests {
    use super::*;
    #[test]
    fn paired_square_brackets() {
        assert!(brackets_are_balanced("[]"));
    }

    #[test]
    fn empty_string() {
        assert!(brackets_are_balanced(""));
    }

    #[test]
    fn unpaired_brackets() {
        assert!(!brackets_are_balanced("[["));
    }

    #[test]
    fn wrong_ordered_brackets() {
        assert!(!brackets_are_balanced("}{"));
    }

    #[test]
    fn wrong_closing_bracket() {
        assert!(!brackets_are_balanced("{]"));
    }

    #[test]
    fn paired_with_whitespace() {
        assert!(brackets_are_balanced("{ }"));
    }

    #[test]
    fn partially_paired_brackets() {
        assert!(!brackets_are_balanced("{[])"));
    }

    #[test]
    fn simple_nested_brackets() {
        assert!(brackets_are_balanced("{[]}"));
    }

    #[test]
    fn several_paired_brackets() {
        assert!(brackets_are_balanced("{}[]"));
    }

    #[test]
    fn paired_and_nested_brackets() {
        assert!(brackets_are_balanced("([{}({}[])])"));
    }

    #[test]
    fn unopened_closing_brackets() {
        assert!(!brackets_are_balanced("{[)][]}"));
    }

    #[test]
    fn unpaired_and_nested_brackets() {
        assert!(!brackets_are_balanced("([{])"));
    }

    #[test]
    fn paired_and_wrong_nested_brackets() {
        assert!(!brackets_are_balanced("[({]})"));
    }

    #[test]
    fn paired_and_incomplete_brackets() {
        assert!(!brackets_are_balanced("{}["));
    }

    #[test]
    fn too_many_closing_brackets() {
        assert!(!brackets_are_balanced("[]]"));
    }

    #[test]
    fn early_incomplete_brackets() {
        assert!(!brackets_are_balanced(")()"));
    }

    #[test]
    fn early_mismatched_brackets() {
        assert!(!brackets_are_balanced("{)()"));
    }

    #[test]
    fn math_expression() {
        assert!(brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"));
    }

    #[test]
    fn complex_latex_expression() {
        let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \

        

          

                 \\end{array}\\right)";

        assert!(brackets_are_balanced(input));
    }
}
