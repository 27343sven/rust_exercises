#[allow(unused)]
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let mut sorted_word = lowercase_word.chars().collect::<Vec<char>>();
    sorted_word.sort_unstable();

    let mut anagrams: HashSet<&'a str> = HashSet::new();
    for anagram in possible_anagrams {
        let lowercase_anagram = anagram.to_lowercase();
        let mut sort_anagram = lowercase_anagram.chars().collect::<Vec<char>>();
        sort_anagram.sort_unstable();
        if sorted_word.eq(&sort_anagram) && lowercase_word != lowercase_anagram {
            anagrams.insert(anagram);
        }
    }
    anagrams
}

fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
    let result = anagrams_for(word, inputs);
    let expected: HashSet<&str> = expected.iter().cloned().collect();
    assert_eq!(result, expected);
}

#[test]
fn test_does_not_detect_a_differently_cased_word_as_its_own_anagram() {
    let word = "banana";
    let inputs = ["bAnana"];
    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn test_does_not_detect_a_differently_cased_unicode_word_as_its_own_anagram() {
    let word = "ΑΒΓ";
    let inputs = ["ΑΒγ"];
    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}
