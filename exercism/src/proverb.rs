use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    list.windows(2)
        .map(|s| format!("For want of a {} the {} was lost.", s[0], s[1]))
        .chain(once(format!("And all for the want of a {}.", list[0])))
        .collect::<Vec<_>>()
        .join("\n")
}

mod tests {
    use super::*;
    #[test]
    fn test_two_pieces() {
        let input = vec!["nail", "shoe"];

        let expected = vec![
            "For want of a nail the shoe was lost.",
            "And all for the want of a nail.",
        ]
        .join("\n");

        assert_eq!(build_proverb(&input), expected);
    }

    // Notice the change in the last line at three pieces.

    #[test]
    fn test_three_pieces() {
        let input = vec!["nail", "shoe", "horse"];

        let expected = vec![
            "For want of a nail the shoe was lost.",
            "For want of a shoe the horse was lost.",
            "And all for the want of a nail.",
        ]
        .join("\n");

        assert_eq!(build_proverb(&input), expected);
    }

    #[test]
    fn test_one_piece() {
        let input = vec!["nail"];

        let expected = String::from("And all for the want of a nail.");

        assert_eq!(build_proverb(&input), expected);
    }

    #[test]
    fn test_zero_pieces() {
        let input: Vec<&str> = vec![];

        let expected = String::new();

        assert_eq!(build_proverb(&input), expected);
    }

    #[test]
    fn test_full() {
        let input = vec![
            "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
        ];

        let expected = vec![
            "For want of a nail the shoe was lost.",
            "For want of a shoe the horse was lost.",
            "For want of a horse the rider was lost.",
            "For want of a rider the message was lost.",
            "For want of a message the battle was lost.",
            "For want of a battle the kingdom was lost.",
            "And all for the want of a nail.",
        ]
        .join("\n");

        assert_eq!(build_proverb(&input), expected);
    }

    #[test]
    fn test_three_pieces_modernized() {
        let input = vec!["pin", "gun", "soldier", "battle"];

        let expected = vec![
            "For want of a pin the gun was lost.",
            "For want of a gun the soldier was lost.",
            "For want of a soldier the battle was lost.",
            "And all for the want of a pin.",
        ]
        .join("\n");

        assert_eq!(build_proverb(&input), expected);
    }
}
