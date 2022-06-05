#[allow(unused)]
use std::sync::mpsc;

use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut end_result = HashMap::new();

    let (tx, rx) = mpsc::channel();
    let mut workers = 0;

    for text in input {
        let clone_tx = tx.clone();
        if workers < worker_count {
            spawn_tread(clone_tx, text);
            workers += 1;
        } else {
            let result = rx.recv().unwrap();
            for (k, v) in result {
                *end_result.entry(k).or_insert(0) += v;
            }
            spawn_tread(clone_tx, text);
        }
    }

    for _ in 1..=workers {
        let result = rx.recv().unwrap();
        for (k, v) in result {
            *end_result.entry(k).or_insert(0) += v;
        }
    }
    end_result
}

fn spawn_tread(tx: mpsc::Sender<HashMap<char, usize>>, word: &str) {
    let test = word.to_string();
    thread::spawn(move || {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for letter in test.to_lowercase().chars() {
            if letter.is_alphabetic() {
                *counts.entry(letter).or_insert(0) += 1;
            }
        }
        tx.send(counts).unwrap();
    });
}

// Poem by Friedrich Schiller. The corresponding music is the European Anthem.

const ODE_AN_DIE_FREUDE: [&str; 8] = [
    "Freude schöner Götterfunken",
    "Tochter aus Elysium,",
    "Wir betreten feuertrunken,",
    "Himmlische, dein Heiligtum!",
    "Deine Zauber binden wieder",
    "Was die Mode streng geteilt;",
    "Alle Menschen werden Brüder,",
    "Wo dein sanfter Flügel weilt.",
];

// Dutch national anthem

const WILHELMUS: [&str; 8] = [
    "Wilhelmus van Nassouwe",
    "ben ik, van Duitsen bloed,",
    "den vaderland getrouwe",
    "blijf ik tot in den dood.",
    "Een Prinse van Oranje",
    "ben ik, vrij, onverveerd,",
    "den Koning van Hispanje",
    "heb ik altijd geëerd.",
];

// American national anthem

const STAR_SPANGLED_BANNER: [&str; 8] = [
    "O say can you see by the dawn's early light,",
    "What so proudly we hailed at the twilight's last gleaming,",
    "Whose broad stripes and bright stars through the perilous fight,",
    "O'er the ramparts we watched, were so gallantly streaming?",
    "And the rockets' red glare, the bombs bursting in air,",
    "Gave proof through the night that our flag was still there;",
    "O say does that star-spangled banner yet wave,",
    "O'er the land of the free and the home of the brave?",
];

#[test]

fn test_no_texts() {
    assert_eq!(frequency(&[], 4), HashMap::new());
}

#[test]

fn test_one_letter() {
    let mut hm = HashMap::new();

    hm.insert('a', 1);

    assert_eq!(frequency(&["a"], 4), hm);
}

#[test]
#[ignore]

fn test_case_insensitivity() {
    let mut hm = HashMap::new();

    hm.insert('a', 2);

    assert_eq!(frequency(&["aA"], 4), hm);
}

#[test]
#[ignore]

fn test_many_empty_lines() {
    let v = vec![""; 1000];

    assert_eq!(frequency(&v[..], 4), HashMap::new());
}

#[test]
#[ignore]

fn test_many_times_same_text() {
    let v = vec!["abc"; 1000];

    let mut hm = HashMap::new();

    hm.insert('a', 1000);

    hm.insert('b', 1000);

    hm.insert('c', 1000);

    assert_eq!(frequency(&v[..], 4), hm);
}
