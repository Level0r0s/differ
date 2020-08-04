// Copyright © 2019 Qtrac Ltd. All rights reserved.

use crate::structs::{Match, Span, Tag};
use fnv::{FnvHashMap, FnvHashSet};
use std::hash::Hash;

/// Provides methods for comparing two sequences.
///
/// See the [crate docs](index.html) for an overview and examples.
pub struct Differ<'a, T>
where
    T: 'a + Sized + Hash + Eq,
{
    a: &'a [T],
    b: &'a [T],
    b2j: FnvHashMap<&'a T, Vec<usize>>,
}

impl<'a, T> Differ<'a, T>
where
    T: 'a + Sized + Hash + Eq,
{
    /// Creates a new `Differ` and computes the comparison data.
    ///
    /// To get all the spans (equals, insertions, deletions, replacements)
    /// necessary to convert sequence `a` into `b`, use
    /// [`spans()`](struct.Differ.html#method.spans).
    ///
    /// To get all the matches (i.e., the positions and lengths) where `a`
    /// and `b` are the same, use
    /// [`matches()`](struct.Differ.html#method.matches).
    ///
    /// If you need _both_ the matches _and_ the spans, use
    /// [`matches()`](struct.Differ.html#method.matches), and then use
    /// [`spans_for_matches()`](spans_for_matches.v.html).
    pub fn new(a: &'a [T], b: &'a [T]) -> Self {
        let mut differ = Differ { a, b, b2j: FnvHashMap::default() };
        differ.chain_b_seq();
        differ
    }

    fn chain_b_seq(&mut self) {
        for i in 0..self.b.len() {
            let indexes =
                self.b2j.entry(&self.b[i]).or_insert_with(Vec::new);
            indexes.push(i);
        }
        let len = self.b.len(); // changed by above
        if len >= 200 {
            let mut b_popular = FnvHashSet::default();
            let test_len = (len as f64 / 100.0).floor() as usize + 1;
            for (element, indexes) in self.b2j.iter() {
                if indexes.len() > test_len {
                    b_popular.insert(*element);
                }
            }
            for element in &b_popular {
                self.b2j.remove(*element);
            }
        }
    }

    /// Returns all the spans (equals, insertions, deletions,
    /// replacements) necessary to convert sequence `a` into `b`.
    ///
    /// If you need _both_ the matches _and_ the spans, use
    /// [`matches()`](struct.Differ.html#method.matches), and then use
    /// [`spans_for_matches()`](spans_for_matches.v.html).
    pub fn spans(&self) -> Vec<Span> {
        let matches = self.matches();
        spans_for_matches(&matches)
    }

    /// Returns every [`Match`](struct.Match.html) between the two
    /// sequences.
    ///
    /// The differences are the spans between matches.
    ///
    /// To get all the spans (equals, insertions, deletions, replacements)
    /// necessary to convert sequence `a` into `b`, use
    /// [`spans()`](struct.Differ.html#method.spans).
    pub fn matches(&self) -> Vec<Match> {
        let a_len = self.a.len();
        let b_len = self.b.len();
        let mut queue = vec![(0, a_len, 0, b_len)];
        let mut matches = vec![];
        while !queue.is_empty() {
            let (a_start, a_end, b_start, b_end) = queue.pop().unwrap();
            let m = self.longest_match(a_start, a_end, b_start, b_end);
            let i = m.a_start;
            let j = m.b_start;
            let k = m.length;
            if k > 0 {
                matches.push(m);
                if a_start < i && b_start < j {
                    queue.push((a_start, i, b_start, j));
                }
                if i + k < a_end && j + k < b_end {
                    queue.push((i + k, a_end, j + k, b_end));
                }
            }
        }
        matches.sort();
        let mut a_start = 0;
        let mut b_start = 0;
        let mut length = 0;
        let mut non_adjacent = vec![];
        for m in &matches {
            if a_start + length == m.a_start
                && b_start + length == m.b_start
            {
                length += m.length
            } else {
                if length != 0 {
                    non_adjacent
                        .push(Match::new(a_start, b_start, length));
                }
                a_start = m.a_start;
                b_start = m.b_start;
                length = m.length;
            }
        }
        if length != 0 {
            non_adjacent.push(Match::new(a_start, b_start, length));
        }
        non_adjacent.push(Match::new(a_len, b_len, 0));
        non_adjacent
    }

    /// Returns the longest [`Match`](struct.Match.html) between the two
    /// given sequences, within the given index ranges.
    ///
    /// This is used internally, but may be useful, e.g., when called
    /// with say, `differ.longest_match(0, a.len(), 0, b.len())`.
    pub fn longest_match(
        &self,
        a_start: usize,
        a_end: usize,
        b_start: usize,
        b_end: usize,
    ) -> Match {
        let mut best_i = a_start;
        let mut best_j = b_start;
        let mut best_size = 0;
        let mut j2len: FnvHashMap<usize, usize> = FnvHashMap::default();
        for i in a_start..a_end {
            let mut new_j2len: FnvHashMap<usize, usize> =
                FnvHashMap::default();
            if let Some(indexes) = self.b2j.get(&self.a[i]) {
                for j in indexes {
                    let j = *j;
                    if j < b_start {
                        continue;
                    };
                    if j >= b_end {
                        break;
                    };
                    let k = match j2len.get(&(j.wrapping_sub(1))) {
                        Some(n) => n + 1,
                        None => 1,
                    };
                    new_j2len.insert(j, k);
                    if k > best_size {
                        best_i = i.wrapping_sub(k).wrapping_add(1);
                        best_j = j.wrapping_sub(k).wrapping_add(1);
                        best_size = k;
                    }
                }
            }
            j2len = new_j2len;
        }
        while best_i > a_start
            && best_j > b_start
            && self.a[best_i - 1] == self.b[best_j - 1]
        {
            best_i -= 1;
            best_j -= 1;
            best_size += 1;
        }
        while best_i + best_size < a_end
            && best_j + best_size < b_end
            && self.a[best_i + best_size] == self.b[best_j + best_size]
        {
            best_size += 1;
        }
        Match::new(best_i, best_j, best_size)
    }
}

/// Returns all the spans (equals, insertions, deletions, replacements)
/// necessary to convert sequence `a` into `b`, given the precomputed
/// matches.
///
/// Use this if you need _both_ matches _and_ spans, to avoid needlessly
/// recomputing the matches, i.e., call
/// [`Differ::matches()`](struct.Differ.html#method.matches) to get the
/// matches, and then this function for the spans.
///
/// If you don't need the matches, then use
/// [`spans()`](struct.Differ.html#method.spans).
pub fn spans_for_matches(matches: &[Match]) -> Vec<Span> {
    let mut spans = vec![];
    let mut i = 0;
    let mut j = 0;
    for m in matches {
        let mut span = Span::equal(i, m.a_start, j, m.b_start);
        if i < m.a_start && j < m.b_start {
            span.tag = Tag::Replace;
        } else if i < m.a_start {
            span.tag = Tag::Delete;
        } else if j < m.b_start {
            span.tag = Tag::Insert;
        }
        if span.tag != Tag::Equal {
            spans.push(span);
        }
        i = m.a_start + m.length;
        j = m.b_start + m.length;
        if m.length != 0 {
            spans.push(Span::equal(m.a_start, i, m.b_start, j));
        }
    }
    spans
}
