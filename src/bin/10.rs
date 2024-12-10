use std::collections::HashMap;
use std::mem::take;

advent_of_code::solution!(10);

fn get_paths(head: (usize, usize), map: &[Vec<u8>], width: usize, height: usize) -> (usize, usize) {
    let mut active_positions = HashMap::new();
    let mut next_positions = HashMap::new();
    
    active_positions.insert((head.0, head.1), 1);
    
    for elevation in 1..=9 {
        for ((row, col), paths) in &active_positions {
            if *row != 0 && map[row - 1][*col] == elevation {
                next_positions.entry((row - 1, *col))
                    .and_modify(|next_paths| {
                        *next_paths += *paths;
                    })
                    .or_insert(*paths);
            }
            if *row != width - 1 && map[row + 1][*col] == elevation { 
                next_positions.entry((row + 1, *col))
                    .and_modify(|next_path| {
                        *next_path += *paths;
                    })
                    .or_insert(*paths);
            }
            if *col != 0 && map[*row][col - 1] == elevation { 
                next_positions.entry((*row, col - 1))
                    .and_modify(|next_path| {
                        *next_path += *paths;
                    })
                    .or_insert(*paths);
            }
            if *col != height - 1 && map[*row][col + 1] == elevation {
                next_positions.entry((*row, col + 1))
                    .and_modify(|next_paths| {
                        *next_paths += *paths;
                    })
                    .or_insert(*paths);
            }
        }
        active_positions = take(&mut next_positions);
    }

    (active_positions.len(), active_positions.iter().map(|(_, paths)| paths).sum())
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim().chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let width = input.first().unwrap().len();
    let height = input.len();
    
    let mut sum = 0;
    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if *cell == 0 { 
                let (p, _) = get_paths((row_idx, col_idx), &input, width, height);
                sum += p;
            }
        }
    }
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {

    let input = parse_input(input);
    let width = input.first().unwrap().len();
    let height = input.len();

    let mut sum = 0;
    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if *cell == 0 {
                let (_, p) = get_paths((row_idx, col_idx), &input, width, height);
                sum += p;
            }
        }
    }
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
