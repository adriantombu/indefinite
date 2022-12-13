use regex::Regex;

pub struct Helper;

impl Helper {
    // TODO: Explore & benchmark other solutions
    pub fn starts_with_vowel(word: &str) -> bool {
        let re = Regex::new(r"^[aeiouAEIOU]").unwrap();

        re.is_match(word)
    }

    pub fn capitalize(word: &str) -> String {
        let mut c = word.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    /// Returns the first word of a string
    pub fn get_first_word(sentence: &str) -> String {
        let re = Regex::new(r"[\s'-]").unwrap();
        let splits = re.split(sentence).take(1).collect::<Vec<_>>();

        splits.first().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_with_vowel_lowercase() {
        assert!(Helper::starts_with_vowel("accept"));
        assert!(Helper::starts_with_vowel("empathetic"));
        assert!(Helper::starts_with_vowel("invulnerable"));
        assert!(Helper::starts_with_vowel("outdoor"));
        assert!(Helper::starts_with_vowel("utopia"));
    }

    #[test]
    fn test_starts_with_vowel_uppercase() {
        assert!(Helper::starts_with_vowel("ACCEPT"));
        assert!(Helper::starts_with_vowel("EMPATHETIC"));
        assert!(Helper::starts_with_vowel("INVULNERABLE"));
        assert!(Helper::starts_with_vowel("OUTDOOR"));
        assert!(Helper::starts_with_vowel("UTOPIA"));
    }

    #[test]
    fn test_starts_with_not_vowel_lowercase() {
        assert!(!Helper::starts_with_vowel(""));
        assert!(!Helper::starts_with_vowel("zombie"));
        assert!(!Helper::starts_with_vowel("kindness"));
        assert!(!Helper::starts_with_vowel("glamorous"));
        assert!(!Helper::starts_with_vowel("041231232"));
        assert!(!Helper::starts_with_vowel("!:&éàè_qsdf"));
    }

    #[test]
    fn test_starts_with_not_vowel_uppercase() {
        assert!(!Helper::starts_with_vowel("ZOMBIE"));
        assert!(!Helper::starts_with_vowel("KINDNESS"));
        assert!(!Helper::starts_with_vowel("GLAMOROUS"));
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(Helper::capitalize(""), "");
        assert_eq!(Helper::capitalize("accept"), "Accept");
        assert_eq!(Helper::capitalize("zombie"), "Zombie");
    }

    #[test]
    fn test_get_first() {
        assert_eq!(&Helper::get_first_word("heir's"), "heir");
        assert_eq!(&Helper::get_first_word("honor-bound"), "honor");
        assert_eq!(&Helper::get_first_word("ouija board"), "ouija");
    }
}
