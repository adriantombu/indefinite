mod helper;
mod irregular;
mod rules;

use crate::helper::Helper;

pub fn indefinite(sentence: &str) -> String {
    format!("{} {}", apply_rules(sentence), sentence)
}

pub fn indefinite_capitalized(sentence: &str) -> String {
    Helper::capitalize(&format!("{} {}", apply_rules(sentence), sentence))
}

pub fn indefinite_article_only(sentence: &str) -> String {
    apply_rules(sentence)
}

pub fn indefinite_article_only_capitalized(sentence: &str) -> String {
    Helper::capitalize(&apply_rules(sentence))
}

fn apply_rules(sentence: &str) -> String {
    let word = &Helper::get_first_word(sentence);

    if rules::number::Number::check(word) {
        rules::number::Number::run(word)
    } else if rules::acronym::Acronym::check(word) {
        rules::acronym::Acronym::run(word)
    } else {
        rules::other::Other::run(word)
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indefinite() {
        assert_eq!(indefinite("Umbrella"), "an Umbrella");
        assert_eq!(indefinite("FFA prodigy"), "an FFA prodigy");
        assert_eq!(indefinite("honor"), "an honor");
        assert_eq!(indefinite("euro"), "a euro");
        assert_eq!(indefinite("ukulele"), "a ukulele");
        assert_eq!(indefinite("ouija board"), "a ouija board");
        assert_eq!(indefinite("honor-bound"), "an honor-bound");
        assert_eq!(indefinite("heir's"), "an heir's");
        assert_eq!(indefinite("red rum"), "a red rum");
        assert_eq!(indefinite("Hour"), "an Hour");
        assert_eq!(indefinite("hours"), "an hours");
        assert_eq!(indefinite("heiresses"), "an heiresses");
        assert_eq!(indefinite("honored"), "an honored");

        assert_eq!(indefinite("11000000000000"), "an 11000000000000");
        assert_eq!(indefinite("110000000000000"), "a 110000000000000");
        assert_eq!(indefinite("18000000"), "an 18000000");
        assert_eq!(indefinite("180000000"), "a 180000000");
        assert_eq!(indefinite("800000"), "an 800000");

        assert_eq!(indefinite("u"), "a u");
        assert_eq!(indefinite("f"), "an f");
        assert_eq!(indefinite("h"), "an h");
        assert_eq!(indefinite("l"), "an l");
        assert_eq!(indefinite("m"), "an m");
        assert_eq!(indefinite("n"), "an n");
        assert_eq!(indefinite("r"), "an r");
        assert_eq!(indefinite("s"), "an s");
        assert_eq!(indefinite("x"), "an x");

        assert_eq!(indefinite("a"), "an a");
        assert_eq!(indefinite("b"), "a b");
    }

    #[test]
    fn test_indefinite_capitalized() {
        assert_eq!(indefinite_capitalized("apple"), "An apple");
        assert_eq!(indefinite_capitalized("banana"), "A banana");
    }

    #[test]
    fn test_indefinite_article_only() {
        assert_eq!(indefinite_article_only("apple"), "an");
        assert_eq!(indefinite_article_only("pear"), "a");
    }

    #[test]
    fn test_indefinite_article_only_capitalized() {
        assert_eq!(indefinite_article_only_capitalized("apple"), "An");
        assert_eq!(indefinite_article_only_capitalized("pear"), "A");
    }
}
