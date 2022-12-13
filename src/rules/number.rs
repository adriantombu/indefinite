use regex::Regex;

pub struct Number;

impl Number {
    /// Checks if the word is a number
    pub fn check(word: &str) -> bool {
        let re = Regex::new(r"^([\d,.]+)").unwrap();

        re.is_match(word)
    }

    pub fn run(word: &str) -> String {
        let re = Regex::new(r"^[,.]").unwrap();
        let number = re.replace_all(word, "");

        if Self::starts_with_height_eleven_or_eighteen(word)
            && (Self::starts_with_eleven_or_eighteen(word) && (number.len() - 2) % 3 == 0
                || word.starts_with('8'))
        {
            "an".to_string()
        } else {
            "a".to_string()
        }
    }

    /// Checks if the word starts with 8, 11 or 18
    fn starts_with_height_eleven_or_eighteen(word: &str) -> bool {
        let re = Regex::new(r"^(8|11|18)").unwrap();

        re.is_match(word)
    }

    /// Checks if the word starts with 11 or 18
    fn starts_with_eleven_or_eighteen(word: &str) -> bool {
        let re = Regex::new(r"^(11|18)").unwrap();

        re.is_match(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        assert!(Number::check("213 horses were running in the wild"));
        assert!(Number::check("4232,96 euro"));
        assert!(Number::check("10231.27 dollars"));
    }

    #[test]
    fn test_not_check() {
        assert!(!Number::check("lorem ipsum is NOT a valid number"));
    }

    #[test]
    fn test_starts_with_height_eleven_or_eighteen() {
        assert!(Number::starts_with_height_eleven_or_eighteen("8123"));
        assert!(Number::starts_with_height_eleven_or_eighteen("11023"));
        assert!(Number::starts_with_height_eleven_or_eighteen("18"));
    }

    #[test]
    fn test_not_starts_with_height_eleven_or_eighteen() {
        assert!(!Number::starts_with_height_eleven_or_eighteen("9123"));
        assert!(!Number::starts_with_height_eleven_or_eighteen("1231"));
        assert!(!Number::starts_with_height_eleven_or_eighteen("745"));
    }

    #[test]
    fn test_starts_with_eleven_or_eighteen() {
        assert!(Number::starts_with_eleven_or_eighteen("11023"));
        assert!(Number::starts_with_eleven_or_eighteen("1831"));
    }

    #[test]
    fn test_not_starts_with_eleven_or_eighteen() {
        assert!(!Number::starts_with_eleven_or_eighteen("8123"));
        assert!(!Number::starts_with_eleven_or_eighteen("9123"));
        assert!(!Number::starts_with_eleven_or_eighteen("1231"));
        assert!(!Number::starts_with_eleven_or_eighteen("745"));
    }

    #[test]
    fn test_run_starts_with_11() {
        assert_eq!(Number::run("11"), "an");
        assert_eq!(Number::run("110"), "a");
        assert_eq!(Number::run("11000"), "an");
        assert_eq!(Number::run("110000"), "a");
        assert_eq!(Number::run("1100000"), "a");
        assert_eq!(Number::run("11000000"), "an");
        assert_eq!(Number::run("110000000"), "a");
        assert_eq!(Number::run("1100000000"), "a");
        assert_eq!(Number::run("11000000000"), "an");
        assert_eq!(Number::run("110000000000"), "a");
        assert_eq!(Number::run("1100000000000"), "a");
        assert_eq!(Number::run("11000000000000"), "an");
        assert_eq!(Number::run("110000000000000"), "a");
        assert_eq!(Number::run("1100000000000000"), "a");
        assert_eq!(Number::run("11000000000000000"), "an");
    }

    #[test]
    fn test_run_starts_with_18() {
        assert_eq!(Number::run("18"), "an");
        assert_eq!(Number::run("180"), "a");
        assert_eq!(Number::run("18000"), "an");
        assert_eq!(Number::run("180000"), "a");
        assert_eq!(Number::run("1800000"), "a");
        assert_eq!(Number::run("18000000"), "an");
        assert_eq!(Number::run("180000000"), "a");
        assert_eq!(Number::run("1800000000"), "a");
        assert_eq!(Number::run("18000000000"), "an");
        assert_eq!(Number::run("180000000000"), "a");
        assert_eq!(Number::run("1800000000000"), "a");
        assert_eq!(Number::run("18000000000000"), "an");
        assert_eq!(Number::run("180000000000000"), "a");
        assert_eq!(Number::run("1800000000000000"), "a");
        assert_eq!(Number::run("18000000000000000"), "an");
    }

    #[test]
    fn test_run_starts_with_8() {
        assert_eq!(Number::run("8"), "an");
        assert_eq!(Number::run("80"), "an");
        assert_eq!(Number::run("8000"), "an");
        assert_eq!(Number::run("80000"), "an");
        assert_eq!(Number::run("800000"), "an");
        assert_eq!(Number::run("8000000"), "an");
        assert_eq!(Number::run("80000000"), "an");
        assert_eq!(Number::run("800000000"), "an");
        assert_eq!(Number::run("8000000000"), "an");
    }

    #[test]
    fn test_run_starts_with_1100_1800() {
        assert_eq!(Number::run("1100"), "a");
        assert_eq!(Number::run("1800"), "a");
    }

    #[test]
    fn test_run_starts_with_other_numbers() {
        assert_eq!(Number::run("17"), "a");
        assert_eq!(Number::run("14"), "a");
    }
}
