use advent_of_code::count_elements;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::iter::successors;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let lines = parse(input);
    let sum = blinks(&lines, 25).values().copied().sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = parse(input);
    let sum = blinks(&lines, 75).values().copied().sum();
    Some(sum)
}

fn blinks(input: &[u64], n: usize) -> FxHashMap<u64, usize> {
    successors(Some(count_elements(input)), |nums| Some(blink(nums)))
        .nth(n)
        .unwrap()
}

fn split(v: u64) -> (u64, u64) {
    let div = 10u64.pow((v.ilog10() + 1) /2);
    (v/ div, v % div)
}

fn blink(input: &FxHashMap<u64, usize>) -> FxHashMap<u64, usize> {
    let mut ret = FxHashMap::with_capacity_and_hasher(input.len(), FxBuildHasher::default());
    for (stone, count) in input {
        if *stone == 0 {
            *ret.entry(1).or_default() += count;
        } else if stone.ilog10() % 2 == 1 {
            let (a, b) = split(*stone);
            *ret.entry(a).or_default() += count;
            *ret.entry(b).or_default() += count;
        } else {
            *ret.entry(*stone * 2024).or_default() += count;
        }
    }
    ret
}

fn parse(input: &str) -> Vec<u64> {
    input.split_whitespace()
        .flat_map(|x| x.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
