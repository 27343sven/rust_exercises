#[allow(dead_code, unused)]

pub fn abbreviate(phrase: &str) -> String {
    phrase.split(|c: char| c.is_whitespace() || c == '-' || c == ',').filter_map(|word| {
        let no_lowercase = word.chars().filter(|c| !c.is_alphabetic() || c.is_uppercase()).collect::<String>();
        if word.len() == no_lowercase.len() || no_lowercase.len() == 0 {
            word.chars().into_iter().filter(|c| c.is_alphabetic()).nth(0).and_then(|n| Some(n.to_uppercase().to_string()))
        } else {
            Some(word.chars().filter(|c| c.is_uppercase()).collect())
        }
    }).collect()
}

mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(abbreviate(""), "");
    }

    #[test]
    fn basic() {
        assert_eq!(abbreviate("Portable Network Graphics"), "PNG");
    }

    #[test]
    fn lowercase_words() {
        assert_eq!(abbreviate("Ruby on Rails"), "ROR");
    }

    #[test]
    fn camelcase() {
        assert_eq!(abbreviate("HyperText Markup Language"), "HTML");
    }

    #[test]
    fn punctuation() {
        assert_eq!(abbreviate("First In, First Out"), "FIFO");
    }

    #[test]
    fn all_caps_word() {
        assert_eq!(
            abbreviate("GNU Image Manipulation Program"),
            "GIMP"
        );
    }

    #[test]
    fn all_caps_word_with_punctuation() {
        assert_eq!(abbreviate("PHP: Hypertext Preprocessor"), "PHP");
    }

    #[test]
    fn punctuation_without_whitespace() {
        assert_eq!(
            abbreviate("Complementary metal-oxide semiconductor"),
            "CMOS"
        );
    }

    #[test]
    fn very_long_abbreviation() {
        assert_eq!(
            abbreviate(
                "Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me"
            ),
            "ROTFLSHTMDCOALM"
        );
    }

    #[test]
    fn consecutive_delimiters() {
        assert_eq!(
            abbreviate("Something - I made up from thin air"),
            "SIMUFTA"
        );
    }

    #[test]
    fn apostrophes() {
        assert_eq!(abbreviate("Halley's Comet"), "HC");
    }

    #[test]
    fn underscore_emphasis() {
        assert_eq!(abbreviate("The Road _Not_ Taken"), "TRNT");
    }
}
