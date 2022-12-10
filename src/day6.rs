use std::collections::BTreeSet;

pub fn signal_slow(b: &[u8], window_len: usize) -> Option<usize> {
    b.windows(window_len)
        .position(|x| {
            let mut set = BTreeSet::new();
            for c in x {
                set.insert(*c);
            }
            set.len() == window_len
        })
        .map(|winner| winner + window_len)
}

#[cfg(test)]
mod tests {
    use crate::day6::signal_slow;

    #[test]
    fn smol_part1() {
        let buf = include_bytes!("../input/day6.smol");
        assert_eq!(signal_slow(buf, 4).unwrap(), 7);
    }

    #[test]
    fn part1() {
        let buf = include_bytes!("../input/day6");
        assert_eq!(signal_slow(buf, 4).unwrap(), 1300);
    }

    #[test]
    fn smol_part2() {
        let buf = include_bytes!("../input/day6.smol");
        assert_eq!(signal_slow(buf, 14).unwrap(), 19);
    }

    #[test]
    fn part2() {
        let buf = include_bytes!("../input/day6");
        assert_eq!(signal_slow(buf, 14).unwrap(), 3986);
    }

    #[test]
    fn howmanywindows() {
        let v: Vec<_> = (0..64).collect();
        assert_eq!(v.windows(14).count(),51);
    }
}