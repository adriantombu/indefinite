use crate::helper::Helper;
use crate::irregular::Irregular;

pub struct Other;

impl Other {
    pub fn run(word: &str) -> String {
        let word = &word.to_lowercase();
        let is_irregular = Irregular::check(word);
        let starts_with_a_vowel = Helper::starts_with_vowel(word);

        // If it starts with a vowel and isn't irregular: "an"
        // If it starts with a vowel and IS irregular: "a"
        // If it starts with a consonant and isn't irregular: "a"
        // If it starts with a consonant and IS irregular: "an"
        if (starts_with_a_vowel || is_irregular) && !(starts_with_a_vowel && is_irregular) {
            "an".to_string()
        } else {
            "a".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        assert_eq!(Other::run("apple"), "an");
        assert_eq!(Other::run("banana"), "a");
        assert_eq!(Other::run("Apple"), "an");
        assert_eq!(Other::run("Banana"), "a");
    }
}
