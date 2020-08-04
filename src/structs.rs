// Copyright © 2019 Qtrac Ltd. All rights reserved.

#[cfg(feature="use_serde")]
use serde_derive::{Deserialize, Serialize};
use std::fmt;

/// Holds the start indexes and length of one match.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature="use_serde", derive(Serialize, Deserialize))]
pub struct Match {
    pub a_start: usize,
    pub b_start: usize,
    pub length: usize,
}

impl Match {
    #[doc(hidden)]
    pub fn new(a_start: usize, b_start: usize, length: usize) -> Match {
        Match { a_start, b_start, length }
    }
}

/// Used in a [`Span`](struct.Span.html) to indicate what kind of span it
/// is.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature="use_serde", derive(Serialize, Deserialize))]
pub enum Tag {
    Equal,
    Insert,
    Delete,
    Replace,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tag = match *self {
            Tag::Equal => "equal",
            Tag::Insert => "insert",
            Tag::Delete => "delete",
            Tag::Replace => "replace",
        };
        write!(f, "{}", tag)
    }
}

/// Holds the data describing one span: what kind of span it is (the
/// [`Tag`](enum.Tag.html)) and the start and end indexes in sequence `a`
/// and `b`.
///
/// The indexes are half-open ranges, so go from the start up to one
/// position before the end.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Span {
    pub tag: Tag,
    pub a_start: usize,
    pub a_end: usize,
    pub b_start: usize,
    pub b_end: usize,
}

impl Span {
    #[doc(hidden)]
    pub fn equal(
        a_start: usize,
        a_end: usize,
        b_start: usize,
        b_end: usize,
    ) -> Span {
        Span { tag: Tag::Equal, a_start, a_end, b_start, b_end }
    }

    #[doc(hidden)]
    pub fn insert(
        a_start: usize,
        a_end: usize,
        b_start: usize,
        b_end: usize,
    ) -> Span {
        Span { tag: Tag::Insert, a_start, a_end, b_start, b_end }
    }

    #[doc(hidden)]
    pub fn delete(
        a_start: usize,
        a_end: usize,
        b_start: usize,
        b_end: usize,
    ) -> Span {
        Span { tag: Tag::Delete, a_start, a_end, b_start, b_end }
    }

    #[doc(hidden)]
    pub fn replace(
        a_start: usize,
        a_end: usize,
        b_start: usize,
        b_end: usize,
    ) -> Span {
        Span { tag: Tag::Replace, a_start, a_end, b_start, b_end }
    }
}
