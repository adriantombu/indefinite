pub const LIST: [&str; 118] = [
    // Nouns: eu like y
    "eunuch",
    "eucalyptus",
    "eugenics",
    "eulogy",
    "euphemism",
    "euphony",
    "euphoria",
    "eureka",
    // Adjectives: eu like y
    "euro",
    "european",
    "euphemistic",
    "euphonic",
    "euphoric",
    // Adverbs: eu like y
    "euphemistically",
    "euphonically",
    "euphorically",
    // Nouns: silent h
    "heir",
    "heiress",
    "herb",
    "homage",
    "honesty",
    "honor",
    "honour",
    "hour",
    // Adjectives: silent h
    "honest",
    "honorous",
    // Adverbs: silent h
    "honestly",
    "hourly",
    // Nouns: o like w
    "one",
    "ouija",
    // Adjectives: o like w
    "once",
    // Nouns: u like y
    "ubiquity",
    "udometer",
    "ufo",
    "uke",
    "ukulele",
    "ululate",
    "unicorn",
    "unicycle",
    "uniform",
    "unify",
    "union",
    "unison",
    "unit",
    "unity",
    "universe",
    "university",
    "upas",
    "ural",
    "uranium",
    "urea",
    "ureter",
    "urethra",
    "urine",
    "urologist",
    "urology",
    "urus",
    "usage",
    "use",
    "user",
    "usual",
    "usurp",
    "usurper",
    "usury",
    "utensil",
    "uterus",
    "utility",
    "utopia",
    "utricle",
    "uvarovite",
    "uvea",
    "uvula",
    "utah",
    "utahn",
    // Adjectives: u like y
    "ubiquitous",
    "ugandan",
    "ukrainian",
    "unanimous",
    "unicameral",
    "unified",
    "unique",
    "unisex",
    "universal",
    "urinal",
    "urological",
    "useful",
    "useless",
    "usurious",
    "utilitarian",
    "utopic",
    // Adverbs: u like y
    "ubiquitously",
    "unanimously",
    "unicamerally",
    "uniquely",
    "universally",
    "urologically",
    "usefully",
    "uselessly",
    "usuriously",
    // Nouns: y like i
    "yttria",
    "yggdrasil",
    "ylem",
    "yperite",
    "ytterbia",
    "ytterbium",
    "yttrium",
    // Adjectives: y like i
    "ytterbous",
    "ytterbic",
    "yttric",
    // Single letters
    "f",
    "h",
    "l",
    "m",
    "n",
    "r",
    "s",
    "u",
    "x",
];

pub struct Irregular;

impl Irregular {
    /// Checks if the word is irregular
    pub fn check(word: &str) -> bool {
        let mut word = word.to_string();

        if word.len() > 1 {
            for end in ["es", "ed", "s"] {
                if word.ends_with(end) {
                    word = word.replace(end, "");

                    if word.len() as isize - end.len() as isize <= 1 {
                        return false;
                    }
                }
            }
        }

        LIST.contains(&word.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        assert!(Irregular::check("honor"));
        assert!(Irregular::check("euro"));
        assert!(Irregular::check("ukulele"));
        assert!(Irregular::check("ouija"));
        assert!(!Irregular::check("red"));
    }
}
