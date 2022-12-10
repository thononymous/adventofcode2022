use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::util;

pub fn pack_halves<'a>(lines: &'a Vec<&str>) -> Vec<(&'a str, &'a str)> {
    lines.iter()
        .copied()
        .map(|l| l.split_at(l.len() / 2))
        .collect()
}

pub fn day3_part2(input: &str) -> usize {
    let lines = util::input_lines_no_whitespace_no_breaks(input);
    let priorities = day3_prio();
    lines.chunks(3).map(|v| {
        assert_eq!(v.len(), 3);
        let (a, b, c): (HashSet<_>, HashSet<_>, HashSet<_>) = v.iter().map(|s| s.as_bytes().into_iter().copied().collect::<HashSet<u8>>()).collect_tuple().unwrap();
        let iv = a.intersection(&b).copied().collect::<HashSet<_>>().intersection(&c).copied().collect::<Vec<_>>();
        assert_eq!(iv.len(), 1);
        priorities.get(&iv[0]).unwrap()
    }).sum()
}

pub fn day3_part1(input: &str) -> usize {
    let lines = util::input_lines_no_whitespace_no_breaks(input);
    let halves = pack_halves(&lines);
    let priorities = day3_prio();
    halves.into_iter().map(|(l, r)| {
        let a = l.as_bytes().iter().copied().collect::<HashSet<u8>>();
        let b = r.as_bytes().iter().copied().collect::<HashSet<u8>>();
        let iv: Vec<_> = a.intersection(&b).copied().collect();
        assert_eq!(iv.len(), 1);
        priorities.get(&iv[0]).unwrap()
    }).sum()
}

fn day3_prio() -> HashMap<u8, usize> {
    let lower = (b'a'..=b'z').map(|c| (c, (c - (b'a' - 1)) as usize));
    let upper = (b'A'..=b'Z').map(|c| (c, (c - (b'A' - 27)) as usize));
    let priorities: HashMap<_, _> = lower.chain(upper).collect();
    priorities
}

#[cfg(test)]
mod tests {
    use crate::day3::{day3_part1, day3_part2};

    #[test]
    fn smol_part1() {
        let input = include_str!("../input/day3.smol");
        assert_eq!(day3_part1(input), 157);
    }

    #[test]
    fn part1() {
        let input = include_str!("../input/day3");
        assert_eq!(day3_part1(input), 8039);
    }

    #[test]
    fn smol_part2() {
        let input = include_str!("../input/day3.smol");
        assert_eq!(day3_part2(input), 70)
    }

    #[test]
    fn part2() {
        let input = include_str!("../input/day3");
        assert_eq!(day3_part2(input), 2510)
    }
}