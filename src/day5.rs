use itertools::Itertools;
use crate::util::input_lines;

pub fn day5(input: &str, rev: bool) -> String {
    let parsed = input_lines(input);
    let mut it = parsed.iter();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let stack_input: Vec<_> = (&mut it).take_while(|&&s| !s.trim().is_empty()).copied().collect();
    for line in stack_input {
        line.as_bytes().chunks(4).enumerate().for_each(|(n, s)| {
            // pad to n + 2 so we leave an empty 0th stack on the front for convenient addressing during moves
            while stacks.len() < n + 2 {
                stacks.push(Vec::new());
            }

            let s = std::str::from_utf8(s).unwrap().trim();

            if s.len() == 0 {} else if s.contains("[") {
                let s = s.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
                assert_eq!(s.len(), 1);
                println!("{} {:?}", n, s);
                stacks.get_mut(n + 1).unwrap().push(s.bytes().nth(0).unwrap() as char);
            }
        });
    };

    assert!(stacks.len() >= 1);
    assert_eq!(stacks[0].len(), 0);

    // invert each stack
    stacks.iter_mut().for_each(|s| {
        s.reverse();
    });

    println!("========");


    for move_ins in it.copied() {
        print_stacks(&stacks);
        println!("========");

        let (_, qty, _, from, _, to) = move_ins.split_whitespace().collect_tuple().unwrap();
        let qty = qty.parse::<usize>().unwrap();
        let from = from.parse::<usize>().unwrap();
        let to = to.parse::<usize>().unwrap();

        let mut src: Vec<char> = (0..qty).map(|_| {
            let v: &mut Vec<char> = stacks.get_mut(from).unwrap();
            v.pop().unwrap()
        }).collect();

        if rev {
            src.reverse();
        }

        let dest: &mut Vec<char> = stacks.get_mut(to).unwrap();

        while let Some(c) = src.pop() {
            dest.push(c)
        }
    }

    print_stacks(&stacks);

    let tops: Vec<String> = stacks.iter().skip(1).map(|s| s.last() ).filter(|s| s.is_some()).map(|s| s.unwrap().to_string() ).collect();
    tops.join("")
}

pub fn print_stacks(stacks: &Vec<Vec<char>>) {
    for (col, v) in stacks.iter().enumerate().skip(1) {
        println!("{} {:?}", col, v);
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::day5;

    #[test]
    fn part1_smol() {
        let s = day5(include_str!("../input/day5.smol"), true);
        assert_eq!(s, "CMZ");
    }

    #[test]
    fn part1() {
        let s = day5(include_str!("../input/day5"), true);
        assert_eq!(s, "JCMHLVGMG");
    }

    #[test]
    fn part2_smol() {
        let s = day5(include_str!("../input/day5.smol"), false);
        assert_eq!(s, "MCD");
    }

    #[test]
    fn part2() {
        let s = day5(include_str!("../input/day5"), false);
        assert_eq!(s, "LVMRWSSPZ");
    }
}