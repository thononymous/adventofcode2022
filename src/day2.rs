use std::collections::HashMap;

pub fn parse_game(input: &str) -> Vec<(u8, u8)> {
    let v: Vec<(_, _)> = input.split("\n").map(|s| s.trim()).map(|s| {
        let t: Vec<_> = s.split(" ").collect();
        (t[0].as_bytes().first().copied().unwrap(), t[1].as_bytes().first().copied().unwrap())
    }).collect();
    v
}

pub fn outcomes_to_plays(games: &Vec<(u8, u8)>, x: &HashMap<(u8, u8), u8>) -> Vec<(u8, u8)> {
    games.iter().copied().map(|(a, o)| {
        let t = (o, a);
        let play = x.get(&t).unwrap();
        println!("{:?} -> {}", t, play);
        (a, *play)
    }).collect()
}

pub fn score_all(games: &Vec<(u8, u8)>, map: &HashMap<u8, u8>) -> u64 {
    let mut score = 0u64;
    for (l, r) in games.iter().copied() {
        let rr = map.get(&r).copied().unwrap();
        score += score_one(l, rr);
    }
    score
}

fn score_one(left: u8, right: u8) -> u64 {
    const WON: u64 = 6;
    const DRAW: u64 = 3;
    const LOST: u64 = 0;
    let outcome = match (right, left) {
        (b'A', b'A') => { DRAW }
        (b'B', b'B') => { DRAW }
        (b'C', b'C') => { DRAW }
        (b'A', b'C') => { WON }
        (b'B', b'A') => { WON }
        (b'C', b'B') => { WON }
        (b'A', b'B') => { LOST }
        (b'B', b'C') => { LOST }
        (b'C', b'A') => { LOST }
        t => { panic!("impossible play {:?}", t); }
    };
    let score = match right {
        b'A' => { 1 + outcome }
        b'B' => { 2 + outcome }
        b'C' => { 3 + outcome }
        _ => { panic!("bad input") }
    };
    println!("{}", score);
    score
}

pub fn day2_part1(input: &str) -> u64 {
    let part1_map = HashMap::from([(b'X', b'A'), (b'Y', b'B'), (b'Z', b'C')]);
    let v = parse_game(input);
    let score = score_all(&v, &part1_map);
    score
}

pub fn day2_part2(input: &str) -> u64 {
    let part2_map = HashMap::from([
        ((b'X', b'A'), b'C'),        // lose
        ((b'X', b'B'), b'A'),
        ((b'X', b'C'), b'B'),
        ((b'Y', b'A'), b'A'),        // draw
        ((b'Y', b'B'), b'B'),
        ((b'Y', b'C'), b'C'),
        ((b'Z', b'A'), b'B'),        // win
        ((b'Z', b'B'), b'C'),
        ((b'Z', b'C'), b'A'),
    ]);
    let part1_map = HashMap::from([
        (b'A', b'A'),
        (b'B', b'B'),
        (b'C', b'C'),
    ]);
    let v = parse_game(input);
    let vv = outcomes_to_plays(&v, &part2_map);
    let score = score_all(&vv, &part1_map);
    score
}

#[cfg(test)]
mod tests {
    use super::{day2_part1, day2_part2};

    #[test]
    fn smol_part1() {
        let input = include_str!("../input/day2.smol");
        let score = day2_part1(input);
        assert_eq!(score, 15)
    }

    #[test]
    fn part1() {
        let input = include_str!("../input/day2");
        let score = day2_part1(input);
        assert_eq!(score, 9759)
    }

    #[test]
    fn smol_part2() {
        let input = include_str!("../input/day2.smol");
        let score = day2_part2(input);
        assert_eq!(score, 12);
    }

    #[test]
    fn part2() {
        let input = include_str!("../input/day2");
        let score = day2_part2(input);
        assert_eq!(score, 12429);
    }
}