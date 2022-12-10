use std::cmp::{max, min};
use std::ops::{Add, Bound, RangeBounds, RangeInclusive, Sub};
use itertools::Itertools;
use crate::util::input_lines_no_whitespace_no_breaks;

use num::{Bounded, NumCast, One, Signed};

// this might not be a fast or experienced way to do it but it gives me practice
// with trait bounds and some of the standard traits, and a convenient syntax for
// specifying ranges

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SimpleInterval {
    start: i64,
    end: i64,
}

#[inline]
pub fn overlaps(a: SimpleInterval, b: SimpleInterval) -> bool {
    let (x1, x2) = (a.start, a.end);
    let (y1, y2) = (b.start, b.end);
    max(x1, y1) <= min(x2, y2)
}

#[inline]
pub fn includes(a: SimpleInterval, b: SimpleInterval) -> bool {
    a.start <= b.start && a.end >= b.end
}

impl SimpleInterval {
    #[inline]
    pub fn overlaps(&self, other: &SimpleInterval) -> bool {
        overlaps(*self, *other)
    }

    #[inline]
    pub fn includes(&self, included: &SimpleInterval) -> bool {
        includes(*self, *included)
    }

    #[inline]
    pub fn included_in(&self, included_in: &SimpleInterval) -> bool {
        includes(*included_in, *self)
    }
}

impl<U> From<RangeInclusive<U>> for SimpleInterval
    where
        U: Signed + Copy + Add<U> + Sub<U> + Bounded + One + NumCast
{
    fn from(value: RangeInclusive<U>) -> Self {
        let start = match value.start_bound() {
            Bound::Included(&i) => { i }
            Bound::Excluded(&e) => { e.add(U::one()) }
            Bound::Unbounded => { U::min_value() }
        }.to_i64().unwrap();

        let end = match value.end_bound() {
            Bound::Included(&i) => { i }
            Bound::Excluded(&e) => { e.sub(U::one()) }
            Bound::Unbounded => { U::max_value() }
        }.to_i64().unwrap();

        Self {
            start,
            end,
        }
    }
}

pub fn day4_part1(input: &str) -> usize {
    let pred = |a: &SimpleInterval, b: &SimpleInterval| {
        a.included_in(&b) || b.included_in(&a)
    };
    process_elve_bullshit(input, pred)
}

pub fn day4_part2(input: &str) -> usize {
    let pred = |a: &SimpleInterval, b: &SimpleInterval| {
        a.overlaps(&b)
    };
    process_elve_bullshit(input, pred)
}

fn process_elve_bullshit(input: &str, pred: fn(&SimpleInterval, &SimpleInterval) -> bool) -> usize {
    let v = input_lines_no_whitespace_no_breaks(input);
    let mut overlaps = 0;
    for line in v {
        let ivs: Vec<_> = line.split(",").map(|s| {
            let t: (_, _) = s.split("-").map(|i| i.parse::<i32>().unwrap()).collect_tuple().unwrap();
            SimpleInterval::from(t.0..=t.1)
        }).collect();

        assert_eq!(ivs.len(), 2);

        overlaps += if pred(&ivs[0], &ivs[1]) {
            1
        } else {
            0
        }
    }
    overlaps
}

#[cfg(test)]
mod tests {
    use crate::day4::{day4_part1, day4_part2, includes, overlaps};

    #[test]
    fn test_overlap() {
        assert!(overlaps((0..=0).into(), (0..=1).into()));  // left edge
        assert!(overlaps((0..=1).into(), (1..=1).into()));  // right edge
        assert!(overlaps((5..=44).into(), (10..=12).into())); // inside
        assert!(overlaps((5..=10).into(), (0..=8).into()));   // over left
        assert!(overlaps((2..=10).into(), (9..=12).into()));   // over right
    }

    #[test]
    fn test_no_overlap() {
        assert!(!overlaps((3..=4).into(), (5..=6).into()));  // adjacent
        assert!(!overlaps((2..=3).into(), (6..=7).into()));  // gap
    }

    #[test]
    fn test_included() {
        assert!(includes((0..=10).into(), (0..=2).into())); // left edge
        assert!(includes((0..=10).into(), (4..=7).into())); // inside
        assert!(includes((0..=10).into(), (7..=10).into())); // right edge
    }

    #[test]
    fn test_not_included() {
        assert!(!includes((0..=5).into(), (2..=8).into()));     // overlap
    }


    #[test]
    fn smol_part1() {
        let input = include_str!("../input/day4.smol");
        assert_eq!(2, day4_part1(input));
    }

    #[test]
    fn part1() {
        let input = include_str!("../input/day4");
        assert_eq!(477, day4_part1(input));
    }

    #[test]
    fn smol_part2() {
        let input = include_str!("../input/day4.smol");
        assert_eq!(4, day4_part2(input));
    }

    #[test]
    fn part2() {
        let input = include_str!("../input/day4");
        assert_eq!(830, day4_part2(input));
    }
}