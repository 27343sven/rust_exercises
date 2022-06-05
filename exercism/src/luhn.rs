#[allow(unused)]

pub fn is_valid(code: &str) -> bool {
    let mut err = Ok(());
    let code_vec = code
        .chars()
        .filter_map(|c| {
            if let Some(n) = c.to_digit(10) {
                Some(Ok(n))
            } else if c.is_whitespace() {
                None
            } else {
                Some(Err("Invalid char"))
            }
        })
        .scan(&mut err, |err, res| match res {
            Ok(o) => Some(o),
            Err(e) => {
                **err = Err(e);
                None
            }
        })
        .collect::<Vec<u32>>();

    if code_vec.len() < 2 || err.is_err() {
        return false;
    }

    let checksum: u32 = code_vec
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, n)| {
            if i % 2 == 1 {
                let new_n = n + n;
                if new_n > 9 {
                    new_n - 9
                } else {
                    new_n
                }
            } else {
                n
            }
        })
        .sum();
    checksum % 10 == 0
}

fn process_valid_case(number: &str, is_luhn_expected: bool) {
    assert_eq!(is_valid(number), is_luhn_expected);
}

#[test]

fn test_valid_number_with_an_even_number_of_digits() {
    process_valid_case("095 245 88", true);
}

#[test]

fn test_a_simple_valid_sin_that_remains_valid_if_reversed() {
    process_valid_case("059", true);
}

#[test]
fn test_more_than_a_single_zero_is_valid() {
    process_valid_case("0000 0", true);
}
