# indefinite

> This crate is a port of the [JavaScript library indefinite](https://github.com/tandrewnichols/indefinite), thanks to the original authors of this library!

Prefix a noun with an indefinite article - a or an - based on whether it begins with a vowel.

## Installation

`cargo add indefinite`

## Usage

```rust
use indefinite::indefinite;

assert_eq!(indefinite("honor"), "an honor");
assert_eq!(indefinite("euro"), "a euro");
assert_eq!(indefinite("ukulele"), "a ukulele");
assert_eq!(indefinite("ouija board"), "a ouija board");
```

```rust
use indefinite::test_indefinite_capitalized;

assert_eq!(indefinite_capitalized("apple"), "An apple");
assert_eq!(indefinite_capitalized("banana"), "A banana");
```

```rust
use indefinite::indefinite_article_only;

assert_eq!(indefinite_article_only("apple"), "an");
assert_eq!(indefinite_article_only("pear"), "a");
```


```rust
use indefinite::test_indefinite_article_only_capitalized;

assert_eq!(indefinite_article_only_capitalized("apple"), "An");
assert_eq!(indefinite_article_only_capitalized("pear"), "A");
```
