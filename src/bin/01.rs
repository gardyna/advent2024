use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = into_vectors(input);
    
    let total_distance: usize = left.iter().sorted()
        .zip(right.iter().sorted())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();
    Some(total_distance as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right): (Vec<usize>, Vec<usize>) = into_vectors(input);
    
    let similarity: usize = left.iter()
        .map(|l| l * right.iter().filter(|r| *r == l).count())
        .sum();
    Some(similarity as u32)
}

fn into_vectors(input: &str) -> (Vec<usize>, Vec<usize>) {
    let (left, right): (Vec<usize>, Vec<usize>) = input.lines()
        .filter_map(|line| line.split_whitespace().next_tuple())
        .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
        .unzip();
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
