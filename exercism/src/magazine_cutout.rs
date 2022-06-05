#![allow(unused)]

use std::{collections::HashMap, ptr::NonNull};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_bucket: HashMap<&str, u32> = HashMap::new();
    for word in magazine {
        match word_bucket.get_mut(word) {
            Some(n) => {
                *n = *n + 1;
            }
            None => {
                word_bucket.insert(word, 1);
            }
        }
    }

    note.into_iter().all(|word| {
        match word_bucket.get_mut(word) {
            Some(n) => {
                if *n > 0 {
                    *n = *n - 1;
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    })
}

#[test]
fn cannot_make_note() {
    let magazine = "two times three is not four"
        .split_whitespace()
        .collect::<Vec<&str>>();
    let note = "two times two is four"
        .split_whitespace()
        .collect::<Vec<&str>>();
    assert_eq!(false, can_construct_note(&magazine, &note));
}

#[test]
fn can_make_note() {
    let magazine = "Astronomer Amy Mainzer spent hours chatting with Leonardo DiCaprio for Netflix's 'Don't Look Up'".split_whitespace().collect::<Vec<&str>>();
    let note = "Amy Mainzer chatting with Leonardo DiCaprio"
        .split_whitespace()
        .collect::<Vec<&str>>();
    assert_eq!(true, can_construct_note(&magazine, &note));
}
