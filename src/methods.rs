use crate::helper::Helper;
use crate::rules;

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
