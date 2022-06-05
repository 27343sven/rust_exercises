#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() {
        if check_sublist(_first_list, _second_list) {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    } else if _first_list.len() > _second_list.len() {
        if check_sublist(_first_list, _second_list) {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        }
    } else {
        if check_sublist(_second_list, _first_list) {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
    }
}

pub fn check_sublist<T: PartialEq>(_first: &[T], _second: &[T]) -> bool {
    if _first.len() == 0 || _second.len() == 0 {
        return if _first.len() == _second.len() || _second.len() == 0 {
            true
        } else {
            false
        };
    }

    let mut start: Option<usize> = None;

    for (i, item) in _first.iter().enumerate() {
        start = match start {
            Some(i) => {
                let mut found = true;
                for (j, item) in _second.into_iter().enumerate() {
                    if let Some(item_2) = _first.get(i + j) {
                        if item != item_2 {
                            found = false;
                            break;
                        }
                    } else {
                        found = false;
                        break;
                    }
                }

                if found {
                    return true;
                } else {
                    None
                }
            }
            _ => start,
        };

        if start.is_none() {
            if _second.get(0) == Some(item) {
                start = Some(i)
            }
        }
    }
    false
}

#[test]
fn partially_matching_sublist_at_start() {
    assert_eq!(Comparison::Sublist, sublist(&[1, 1, 2], &[1, 1, 1, 2]));
}

#[test]
fn recurring_values_unequal() {
    assert_eq!(
        Comparison::Unequal,
        sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1])
    );
}

#[test]
fn test_empty_is_a_sublist_of_anything() {
    assert_eq!(Comparison::Sublist, sublist(&[], &['a', 's', 'd', 'f']));
}

#[test]
fn test_anything_is_a_superlist_of_empty() {
    assert_eq!(Comparison::Superlist, sublist(&['a', 's', 'd', 'f'], &[]));
}

#[test]
fn empty_equals_empty() {
    let v: &[u32] = &[];

    assert_eq!(Comparison::Equal, sublist(v, v));
}
