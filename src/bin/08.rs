use std::collections::{HashMap, HashSet};
pub use itertools::{Itertools};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut antinodes = HashSet::new();
    let (map, width, height) = extract_map(input);
    
    for value in map.clone().values() {
        for i in 0..value.len() {
            for j in (i + 1)..value.len() {
                let (a, b) = value[i];
                let (c, d) = value[j];
                let slope = (d as f64 - b as f64) / (c as f64 - a as f64);
                let remainder = b as f64 - a as f64 * slope;
                let mut y1 = b.min(d) as isize;
                let mut y2 = b.max(d) as isize;
                let y_spacing = y2 - y1;
                y1 -= y_spacing;
                y2 += y_spacing;
                
                if y1 >= 0 { 
                    let x1 = ((y1 as f64 - remainder) / slope).round() as isize;
                    if 0 <= x1 && x1 < width as isize { 
                        antinodes.insert((x1, y1));
                    }
                }
                if y2 < height as isize { 
                    let x2 = ((y2 as f64 - remainder) / slope).round() as isize;
                    if 0 <= x2 && x2 < width as isize {
                        antinodes.insert((x2, y2));
                    }
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

fn extract_map(input: &str) -> (HashMap<char, Vec<(usize, usize)>>, usize, usize) {
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let input: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let width = input[0].len();
    let height = input.len();
    for i in 0..height {
        for j in 0..width {
            if input[i][j] != '.' {
                map.entry(input[i][j])
                    .and_modify(|pos| {
                        if !pos.contains(&(j, i)) {
                            pos.push((j, i));
                        }
                    })
                    .or_insert(vec![(j, i)]);
            }
        }
    }
    (map, width, height)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, width, height) = extract_map(input);
    let mut antinodes =HashSet::new();
    
    for values in map.clone().values() {
        for i in 0..values.len() {
            for j in (i + 1)..values.len() {
                let (a, b) = values[i];
                let (c, d) = values[j];
                let slope = (d as f64 - b as f64) / (c as f64 - a as f64);
                let remainder = b as f64 - a as f64 * slope;
                let mut y1 = b.min(d) as isize;
                let mut y2 = b.max(d) as isize;
                let y_spacing = y2 - y1;

                while y1 >= 0 {
                    let x1 = ((y1 as f64 - remainder) / slope).round() as isize;
                    if 0 <= x1 && x1 < width as isize {
                        antinodes.insert((x1, y1));
                    }
                    y1 -= y_spacing;
                }
                while y2 < height as isize {
                    let x2 = ((y2 as f64 - remainder) / slope).round() as isize;
                    if 0 <= x2 && x2 < width as isize { 
                        antinodes.insert((x2, y2));
                    }
                    y2 += y_spacing;
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
