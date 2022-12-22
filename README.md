[![crates.io](https://img.shields.io/crates/v/indefinite.svg)](https://crates.io/crates/indefinite) [![docs.rs](https://img.shields.io/docsrs/indefinite/latest.svg)](https://docs.rs/indefinite/latest/indefinite/index.html)

# indefinite

> This crate is a port of the [JavaScript library indefinite](https://github.com/tandrewnichols/indefinite), thanks to the original authors of this library!

Prefix a noun with an indefinite article - a or an - based on whether it begins with a vowel.

## Installation

`cargo add indefinite`

## Usage

```rust
use indefinite::*;

fn main () {
    assert_eq!(indefinite("honor"), "an honor");
    assert_eq!(indefinite("ouija board"), "a ouija board");

    assert_eq!(indefinite_capitalized("apple"), "An apple");
    assert_eq!(indefinite_capitalized("banana"), "A banana");    

    assert_eq!(indefinite_article_only("apple"), "an");
    assert_eq!(indefinite_article_only("pear"), "a");

    assert_eq!(indefinite_article_only_capitalized("apple"), "An");
    assert_eq!(indefinite_article_only_capitalized("pear"), "A");  
}
```

## Macros

Macros are available as a feature. They are working the same as the functions, except they also accept a list of inputs.

```toml
indefinite = { version = "0.1", features = [ "macros" ] }
```

```rust
use indefinite::*;

fn main () {
    assert_eq!(indefinite!("honor"), "an honor");
    assert_eq!(indefinite!("honor", "euro"), ["an honor", "a euro"]);

    assert_eq!(indefinite_capitalized!("apple"), "An apple");
    assert_eq!(indefinite_capitalized!("banana", "pear"), ["A banana", "A pear"]); 

    assert_eq!(indefinite_article_only!("apple"), "an");
    assert_eq!(indefinite_article_only!("apple","pear"), ["an", "a"]);

    assert_eq!(indefinite_article_only_capitalized!("apple"), "An");
    assert_eq!(indefinite_article_only_capitalized!("apple", "pear"), ["An", "A"]);
}
```
