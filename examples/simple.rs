// Copyright © 2019 Qtrac Ltd. All rights reserved.

use differ::{Differ, Tag};
use std::hash::{Hash, Hasher};

fn main() {
    println!("compare lines:");
    compare_lines();
    println!("\ncompare items:");
    compare_items();
}

fn compare_lines() {
    let a = "foo\nbar\nbaz\nquux".lines().collect::<Vec<_>>();
    let b = "foo\nbaz\nbar\nquux".lines().collect::<Vec<_>>();
    let differ = Differ::new(&a, &b);
    for span in differ.spans() {
        match span.tag {
            Tag::Equal => print_lines('=', &a[span.a_start..span.a_end]),
            Tag::Insert => print_lines('+', &b[span.b_start..span.b_end]),
            Tag::Delete => print_lines('-', &a[span.a_start..span.a_end]),
            Tag::Replace => {
                print_lines('%', &b[span.b_start..span.b_end])
            }
        }
    }
}

fn print_lines(c: char, lines: &[&str]) {
    for line in lines {
        println!("{} {}", c, line);
    }
}

#[derive(Debug, Clone)]
struct Item<'a> {
    // Can have any data
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

fn compare_items() {
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
}
