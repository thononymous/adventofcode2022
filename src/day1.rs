use crate::util::glom_emptyline_delimited;

/// uses emptyline_delimited to get the input in blocks of lines
/// converts each line to a number and sums them
/// returns the largest sum
pub fn day1_part1(filename: &str) -> Result<usize, String> {
    let vv = glom_emptyline_delimited(filename)?;
    let sums = day1_sums(vv);
    let max = sums.iter().max().cloned().unwrap();      // get the max
    Ok(max)
}

/// same as part1 but returns sum of top3 instead of top1
pub fn day1_part2(filename: &str) -> Result<usize, String> {
    let vv = glom_emptyline_delimited(filename)?;
    let mut sums = day1_sums(vv);
    sums.sort();
    sums.reverse();
    let sum3 = sums.iter().take(3).sum();
    Ok(sum3)
}

/// helper for converting newline delimited chunks into sums
fn day1_sums(vv: Vec<Vec<String>>) -> Vec<usize> {
    let sums: Vec<usize> = vv.iter()
        .map(|vs|
            vs.iter()
                .map(|s| s.parse::<usize>().unwrap())       // convert line to number
                .sum())     // inner sum
        .collect();     // collect into vec of sums
    sums
}

#[cfg(test)]
mod tests {
    use super::{day1_part1, day1_part2};

    #[test]
    fn smol_part1() -> Result<(), String> {
        let filename = "input/day1.smol";
        let max = day1_part1(filename)?;
        assert_eq!(max, 24000);
        Ok(())
    }

    #[test]
    fn part1() -> Result<(), String> {
        let filename = "input/day1";
        let max = day1_part1(filename)?;
        assert_eq!(max, 67016);
        Ok(())
    }

    #[test]
    fn smol_part2() -> Result<(), String> {
        let filename = "input/day1.smol";
        let sum3 = day1_part2(filename)?;
        assert_eq!(sum3, 45000);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), String> {
        let filename = "input/day1";
        let sum3 = day1_part2(filename)?;
        assert_eq!(sum3, 200116);
        Ok(())
    }
}
