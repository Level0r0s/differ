// Copyright © 2019 Qtrac Ltd. All rights reserved.

/*!

Differ is a library for finding the differences between two sequences.

The sequences can be vectors of lines, strings (e.g., words), characters,
bytes, or of any custom “item” type so long as it implements the `Hash`
and `Eq` traits (and uses the same data for both).

The simplest use case is first to create a new
[`Differ`](struct.Differ.html) using
[`Differ::new()`](struct.Differ.html#method.new). Then iterate over the
[`Span`](struct.Span.html)s returned by
[`Differ::spans()`](struct.Differ.html#method.spans). These spans show how
to transform sequence `a` into sequence `b` using insertions, deletions,
and replacements.

`differ` is based on the Python
[`difflib.SequenceMatcher`](https://docs.python.org/3/library/difflib.html#difflib.SequenceMatcher).

# Dependencies

To use differ, add this line your `Cargo.toml` file's `[dependencies]`
section:

```toml,ignore
differ = "1"
```

If you want to be able to serialize or deserialize
[`Match`](struct.Match.html)es, [`Span`](struct.Span.html)s, or
[`Tag`](struct.Tag.html)s, then use:

```toml,ignore
differ = { version = "1", features = ["use_serde"] }
```

Then, in your crate root, for Rust 2015 add `extern crate differ`, and for
Rust 2018 add `use differ`.

# Examples

These examples are in the file `examples/simple.rs`. For more examples see
`src/tests.rs`.

## Comparing Text

Here lines are compared, but it could just as easily be strings (e.g.,
words by splitting on whitespace), or characters or bytes.

```
use differ::{Differ, Tag};

let a = "foo\nbar\nbaz\nquux".lines().collect::<Vec<_>>();
let b = "foo\nbaz\nbar\nquux".lines().collect::<Vec<_>>();
let differ = Differ::new(&a, &b);
for span in differ.spans() {
    match span.tag {
        Tag::Equal => print_lines('=', &a[span.a_start..span.a_end]),
        Tag::Insert => print_lines('+', &b[span.b_start..span.b_end]),
        Tag::Delete => print_lines('-', &a[span.a_start..span.a_end]),
        Tag::Replace => print_lines('%', &b[span.b_start..span.b_end]),
    }
}

fn print_lines(c: char, lines: &[&str]) {
    for line in lines {
        println!("{} {}", c, line);
    }
}
```

Output:
```text
= foo
+ baz
= bar
- baz
= quux
```

## Comparing Custom Items

```
use differ::{Differ, Tag};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct Item<'a> { // Can have any data
    x: i32,
    y: i32,
    text: &'a str, // For this example we've decided to compare the text
}

impl<'a> Item<'a> {
    fn new(x: i32, y: i32, text: &'a str) -> Item<'a> {
        Item { x, y, text }
    }
}

impl<'a> Hash for Item<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text.hash(state); // MUST use same data as PartialEq
    }
}

impl<'a> PartialEq for Item<'a> {
    fn eq(&self, other: &Item) -> bool {
        self.text == other.text // MUST use same data as Hash
    }
}
impl<'a> Eq for Item<'a> {}

let a = vec![
    Item::new(1, 3, "quebec"),
    Item::new(2, 4, "alpha"),
    Item::new(3, 8, "bravo"),
    Item::new(5, 9, "x-ray"),
];
let b = vec![
    Item::new(3, 1, "alpha"),
    Item::new(8, 3, "bravo"),
    Item::new(9, 5, "yankee"),
    Item::new(8, 3, "charlie"),
];
let differ = Differ::new(&a, &b);
for span in differ.spans() {
    match span.tag {
        Tag::Equal => (), // ignore
        Tag::Insert => println!("{:?}", span),
        Tag::Delete => println!("{:?}", span),
        Tag::Replace => println!("{:?}", span),
    }
}
```

Output:
```text
Span { tag: Delete, a_start: 0, a_end: 1, b_start: 0, b_end: 0 }
Span { tag: Replace, a_start: 3, a_end: 4, b_start: 2, b_end: 4 }
```

# Upgrading

To upgrade from 0.3._x_ to 1._x_, change method calls to
`spans_for_matches()` to function calls to 
[`differ::spans_for_matches()`](spans_for_matches.v.html).

# License

Differ is free open source software (FOSS) licensed under the GNU
General Public License version 3 (GPLv3).
*/

mod differ;
mod structs;
#[cfg(test)]
mod tests;

pub use crate::differ::{Differ, spans_for_matches};
pub use crate::structs::{Match, Span, Tag};
