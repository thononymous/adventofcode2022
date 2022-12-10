use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;

/// read entire input into a vec of lines/strings
/// afaik aoc challenges have always had input that was small enough to fit in memory
pub fn glom_text(p: &str) -> Result<Vec<String>, String> {
    let input_path = Path::new(p);
    let fh = BufReader::new(File::open(input_path).map_err(|e| e.to_string())?);
    let mut lines = fh.lines();
    let mut v = Vec::new();
    while let Some(r) = lines.next() {
        v.push(r.map_err(|e| e.to_string())?);
    }
    Ok(v)
}

/// takes filename, reads lines, and groups the lines by empty-line delimited chunks
/// returns a Vec<Vec<String>> of those chunks
pub fn glom_emptyline_delimited(p: &str) -> Result<Vec<Vec<String>>, String> {
    let v = glom_text(p)?;
    let mut vv = Vec::new();
    for (key, group) in &v.into_iter().group_by(|e| e.is_empty()) {
        if key {
            continue;
        }
        let gg: Vec<String> = group.collect();
        vv.push(gg);
    }
    Ok(vv)
}

pub fn rdtsc() -> u64 {
    unsafe { std::arch::x86_64::_rdtsc() }
}


pub fn input_lines_no_whitespace_no_breaks(input: &str) -> Vec<&str> {
    input.split_whitespace()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .collect()
}

pub fn input_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

#[cfg(test)]
mod tests {
    use crate::util::glom_emptyline_delimited;

    #[test]
    fn day1_smol_emptyline_delimited() -> Result<(), String> {
        let vv = glom_emptyline_delimited("input/day1.smol")?;
        assert_eq!(vv.len(), 5);
        Ok(())
    }
}
