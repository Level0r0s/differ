// Copyright © 2019 Qtrac Ltd. All rights reserved.

#[cfg(test)]
mod tests {
    use crate::{Differ, Match, Span, spans_for_matches, Tag};

    #[test]
    fn t01() {
        let a = "the quick brown fox jumped over the lazy dogs";
        let b = "the quick red fox jumped over the very busy dogs";
        let a_words = a.split_whitespace().collect::<Vec<_>>();
        let b_words = b.split_whitespace().collect::<Vec<_>>();
        let differ = Differ::new(&a_words, &b_words);
        let expected = vec![
            Span::equal(0, 2, 0, 2),   // the quick
            Span::replace(2, 3, 2, 3), // brown -> red
            Span::equal(3, 7, 3, 7),   // fox jumped over the
            Span::replace(7, 8, 7, 9), // lazy -> very busy
            Span::equal(8, 9, 9, 10),  // dogs
        ];
        for (i, span) in differ.spans().iter().enumerate() {
            if i == 4 {
                assert_eq!(span.a_start, 8);
                assert_eq!(span.a_end, 9);
                assert_eq!(span.b_start, 9);
                assert_eq!(span.b_end, 10);
            }
            assert_eq!(span, &expected[i]);
        }
    }

    #[test]
    fn t02() {
        let a = "qabxcd";
        let b = "abycdf";
        let a_chars = a.chars().collect::<Vec<_>>();
        let b_chars = b.chars().collect::<Vec<_>>();
        let differ = Differ::new(&a_chars, &b_chars);
        let expected = vec![
            Span::delete(0, 1, 0, 0),  // q ->
            Span::equal(1, 3, 0, 2),   // ab
            Span::replace(3, 4, 2, 3), // x -> y
            Span::equal(4, 6, 3, 5),   // cd
            Span::insert(6, 6, 5, 6),  // -> f
        ];
        for (i, span) in differ.spans().iter().enumerate() {
            assert_eq!(span, &expected[i]);
        }
    }

    #[test]
    fn t03() {
        let a = b"private Thread currentThread;";
        let b = b"private volatile Thread currentThread;";
        let differ = Differ::new(&a[..], &b[..]);
        let expected = vec![
            Span::equal(0, 6, 0, 6),    // privat
            Span::insert(6, 6, 6, 15),  // -> e volatil
            Span::equal(6, 29, 15, 38), // e Thread currentThread;
        ];
        for (i, span) in differ.spans().iter().enumerate() {
            assert_eq!(span, &expected[i]);
        }
    }

    #[test]
    fn t04() {
        let a = "the quick brown fox jumped over the lazy dogs";
        let b = "the quick red fox jumped over the very busy dogs";
        let a_words = a.split_whitespace().collect::<Vec<_>>();
        let b_words = b.split_whitespace().collect::<Vec<_>>();
        let differ = Differ::new(&a_words, &b_words);
        let longest =
            differ.longest_match(0, a_words.len(), 0, b_words.len());
        assert_eq!(Match::new(3, 3, 4), longest);
    }

    #[test]
    fn t05() {
        let a = "a s c ( 99 ) x z";
        let b = "r s b c ( 99 )";
        let a_words = a.split_whitespace().collect::<Vec<_>>();
        let b_words = b.split_whitespace().collect::<Vec<_>>();
        let differ = Differ::new(&a_words, &b_words);
        let longest =
            differ.longest_match(0, a_words.len(), 0, b_words.len());
        assert_eq!(Match::new(2, 3, 4), longest);
    }

    #[test]
    fn t06() {
        let a = "foo\nbar\nbaz\nquux";
        let b = "foo\nbaz\nbar\nquux";
        let a_lines = a.lines().collect::<Vec<_>>();
        let b_lines = b.lines().collect::<Vec<_>>();
        let differ = Differ::new(&a_lines, &b_lines);
        let expected = vec![
            Span::equal(0, 1, 0, 1),  // foo
            Span::insert(1, 1, 1, 2), // -> baz
            Span::equal(1, 2, 2, 3),  // bar
            Span::delete(2, 3, 3, 3), // baz ->
            Span::equal(3, 4, 3, 4),  // quux
        ];
        for (i, span) in differ.spans().iter().enumerate() {
            assert_eq!(span, &expected[i]);
        }
    }

    #[test]
    fn t07() {
        let a = "foo\nbar\nbaz\nquux";
        let b = "foo\nbaz\nbar\nquux";
        let a_lines = a.lines().collect::<Vec<_>>();
        let b_lines = b.lines().collect::<Vec<_>>();
        let differ = Differ::new(&a_lines, &b_lines);
        let expected = vec![
            Span::insert(1, 1, 1, 2), // -> baz
            Span::delete(2, 3, 3, 3), // baz ->
        ];
        let mut i = 0;
        for span in differ.spans() {
            // See t08 for better way
            if span.tag != Tag::Equal {
                assert_eq!(&span, &expected[i]);
                i += 1;
            }
        }
    }

    #[test]
    fn t08() {
        let a = "foo\nbar\nbaz\nquux";
        let b = "foo\nbaz\nbar\nquux";
        let a_lines = a.lines().collect::<Vec<_>>();
        let b_lines = b.lines().collect::<Vec<_>>();
        let differ = Differ::new(&a_lines, &b_lines);
        let expected = vec![
            Span::insert(1, 1, 1, 2), // -> baz
            Span::delete(2, 3, 3, 3), // baz ->
        ];
        let mut i = 0;
        for span in differ.spans().iter().filter(|s| s.tag != Tag::Equal)
        {
            assert_eq!(span, &expected[i]);
            i += 1;
        }
    }

    #[test]
    fn t09() {
        let a = &[1, 2, 3, 4, 5, 6];
        let b = &[2, 3, 5, 7];
        let differ = Differ::new(&a[..], &b[..]);
        let matches = differ.matches();
        let expected = vec![
            Span::delete(0, 1, 0, 0),  // 1 ->
            Span::equal(1, 3, 0, 2),   // 2 3
            Span::delete(3, 4, 2, 2),  // 4 ->
            Span::equal(4, 5, 2, 3),   // 5
            Span::replace(5, 6, 3, 4), // 6 -> 7
        ];
        for (i, span) in
            spans_for_matches(&matches).iter().enumerate()
        {
            assert_eq!(span, &expected[i]);
        }
    }

    #[test]
    fn t10() {
        let a = "qabxcd".chars().collect::<Vec<_>>();
        let b = "abycdf".chars().collect::<Vec<_>>();
        let differ = Differ::new(&a, &b);
        let expected = vec![
            Span::delete(0, 1, 0, 0),  // q ->
            Span::equal(1, 3, 0, 2),   // a b
            Span::replace(3, 4, 2, 3), // x -> y
            Span::equal(4, 6, 3, 5),   // c d
            Span::insert(6, 6, 5, 6),  // -> f
        ];
        for (i, span) in differ.spans().iter().enumerate() {
            assert_eq!(span, &expected[i]);
        }
    }

    #[derive(Debug, Clone)]
    struct Item<'a> {
        x: i32,
        y: i32,
        text: &'a str,
    }

    impl<'a> Item<'a> {
        fn new(x: i32, y: i32, text: &'a str) -> Item<'a> {
            Item { x, y, text }
        }
    }

    use std::hash::{Hash, Hasher};

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

    #[test]
    fn t11() {
        let a = vec![
            Item::new(1, 3, "A"),
            Item::new(2, 4, "B"),
            Item::new(3, 8, "C"),
            Item::new(5, 9, "D"),
            Item::new(7, 2, "E"),
            Item::new(3, 8, "F"),
            Item::new(1, 6, "G"),
        ];
        let b = vec![
            Item::new(3, 1, "A"),
            Item::new(8, 3, "C"),
            Item::new(9, 5, "B"),
            Item::new(8, 3, "D"),
            Item::new(6, 1, "E"),
            Item::new(4, 2, "G"),
        ];
        let differ = Differ::new(&a, &b);
        let expected = vec![
            Span::equal(0, 1, 0, 1),  // A
            Span::insert(1, 1, 1, 2), // -> C
            Span::equal(1, 2, 2, 3),  // B
            Span::delete(2, 3, 3, 3), // C ->
            Span::equal(3, 5, 3, 5),  // D E
            Span::delete(5, 6, 5, 5), // F ->
            Span::equal(6, 7, 5, 6),  // G
        ];
        for (i, span) in differ.spans().iter().enumerate() {
            assert_eq!(span, &expected[i]);
        }
    }

    #[test]
    fn t12() {
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
        let expected = vec![
            Span::delete(0, 1, 0, 0),  // quebec ->
            Span::equal(1, 3, 0, 2),   // alpha bravo
            Span::replace(3, 4, 2, 4), // x-ray -> yankee charlie
        ];
        for (i, span) in differ.spans().iter().enumerate() {
            assert_eq!(span, &expected[i]);
        }
    }

    #[test]
    fn t13() {
        let a = "abxcd".chars().collect::<Vec<_>>();
        let b = "abcd".chars().collect::<Vec<_>>();
        let expected = vec![
            Match::new(0, 0, 2),
            Match::new(3, 2, 2),
            Match::new(5, 4, 0),
        ];
        let differ = Differ::new(&a, &b);
        for (i, m) in differ.matches().iter().enumerate() {
            assert_eq!(m, &expected[i]);
        }
    }

    #[test]
    fn t14() {
        let a = "the quick brown fox jumped over the lazy dogs";
        let b = "";
        let a_words = a.split_whitespace().collect::<Vec<_>>();
        let b_words = b.split_whitespace().collect::<Vec<_>>();
        let expected = vec![
            Span::delete(0, 9, 0, 0),
        ];
        let differ = Differ::new(&a_words, &b_words);
        for (i, m) in differ.spans().iter().enumerate() {
            assert_eq!(m, &expected[i]);
        }
    }

    #[test]
    fn t15() {
        let a = "";
        let b = "the quick red fox jumped over the very busy dogs";
        let a_words = a.split_whitespace().collect::<Vec<_>>();
        let b_words = b.split_whitespace().collect::<Vec<_>>();
        let expected = vec![
            Span::insert(0, 0, 0, 10),
        ];
        let differ = Differ::new(&a_words, &b_words);
        for (i, m) in differ.spans().iter().enumerate() {
            assert_eq!(m, &expected[i]);
        }
    }

    #[test]
    fn t16() {
        let a = "";
        let b = "";
        let a_words = a.split_whitespace().collect::<Vec<_>>();
        let b_words = b.split_whitespace().collect::<Vec<_>>();
        let differ = Differ::new(&a_words, &b_words);
        assert_eq!(differ.spans().len(), 0);
    }
}
