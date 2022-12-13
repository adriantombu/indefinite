use crate::helper::Helper;
use regex::Regex;

pub struct Acronym;

impl Acronym {
    /// Checks if the word is an acronym
    pub fn check(word: &str) -> bool {
        word.to_uppercase().eq(word)
    }

    /// If it starts with U: "a"
    /// If it starts with any other vowel: "an"
    /// If it starts with F, H, L, M, N, R, S, or X: "an"
    /// If it starts with any other consonant: "a"
    pub fn run(word: &str) -> String {
        if Self::is_irregular(word) == Helper::starts_with_vowel(word) {
            "a".to_string()
        } else {
            "an".to_string()
        }
    }

    /// Checks if the acronym is irregular
    fn is_irregular(word: &str) -> bool {
        let re = Regex::new(r"^[UFHLMNRSX]+").unwrap();

        re.is_match(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        assert!(Acronym::check("NHL"));
        assert!(Acronym::check("SSI"));
    }

    #[test]
    fn test_is_not_acronym() {
        assert!(!Acronym::check("Umbrella"));
        assert!(!Acronym::check("nhl"));
        assert!(!Acronym::check("ssi"));
    }

    #[test]
    fn test_is_irregular() {
        assert!(Acronym::is_irregular("URSSAF"));
        assert!(Acronym::is_irregular("FIFA"));
        assert!(Acronym::is_irregular("NHL"));
        assert!(Acronym::is_irregular("NASA"));
    }

    #[test]
    fn test_is_not_irregular() {
        assert!(!Acronym::is_irregular("WHO"));
    }

    #[test]
    fn test_run() {
        assert_eq!(&Acronym::run("URSSAF"), "a");
        assert_eq!(&Acronym::run("EULA"), "an");
        assert_eq!(&Acronym::run("XLM"), "an");
        assert_eq!(&Acronym::run("CMYK"), "a");
        assert_eq!(&Acronym::run("IOU"), "an");
        assert_eq!(&Acronym::run("UFO"), "a");
        assert_eq!(&Acronym::run("CEO"), "a");
    }
}
