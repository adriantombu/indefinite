/// Returns the input string with the article related to the first word (also accepts a list of input string)
///
/// # Examples
///
/// ```
/// use indefinite::indefinite;
///
/// assert_eq!(indefinite!("honor"), "an honor");
/// assert_eq!(indefinite!("honor", "euro"), ["an honor", "a euro"]);
/// ```
#[macro_export]
macro_rules! indefinite {
    ($word:literal) => {
        $crate::indefinite($word)
    };

    ($($word:literal),+) => {{
        [$($crate::indefinite($word),)+]
    }};
}

/// Returns the input string with the article related to the first word with the first letter capitalised (also accepts a list of input string)
///
/// # Examples
///
/// ```
/// use indefinite::indefinite_capitalized;
///
/// assert_eq!(indefinite_capitalized!("apple"), "An apple");
/// assert_eq!(indefinite_capitalized!("banana", "pear"), ["A banana", "A pear"]);
/// ```
#[macro_export]
macro_rules! indefinite_capitalized {
    ($word:literal) => {
        $crate::indefinite_capitalized($word)
    };

    ($($word:literal),+) => {{
        [$($crate::indefinite_capitalized($word),)+]
    }};
}

/// Returns only the article related to the first word (also accepts a list of input string)
///
/// # Examples
///
/// ```
/// use indefinite::indefinite_article_only;
///
/// assert_eq!(indefinite_article_only!("apple"), "an");
/// assert_eq!(indefinite_article_only!("apple","pear"), ["an", "a"]);
/// ```
#[macro_export]
macro_rules! indefinite_article_only {
    ($word:literal) => {
        $crate::indefinite_article_only($word)
    };

    ($($word:literal),+) => {{
        [$($crate::indefinite_article_only($word),)+]
    }};
}

/// Returns only the article related to the first word with the first letter capitalised (also accepts a list of input string)
///
/// # Examples
///
/// ```
/// use indefinite::indefinite_article_only_capitalized;
///
/// assert_eq!(indefinite_article_only_capitalized!("apple"), "An");
/// assert_eq!(indefinite_article_only_capitalized!("apple", "pear"), ["An", "A"]);
/// ```
#[macro_export]
macro_rules! indefinite_article_only_capitalized {
    ($word:literal) => {
        $crate::indefinite_article_only_capitalized($word)
    };

    ($($word:literal),+) => {{
        [$($crate::indefinite_article_only_capitalized($word),)+]
    }};
}
