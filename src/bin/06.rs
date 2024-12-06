use std::collections::HashSet;

advent_of_code::solution!(6);

const DIRS: &[(i32, i32)] = &[(-1, 0), (0, 1),(1, 0),(0, -1)];

pub fn part_one(input: &str) -> Option<u32> {
    let (mut matrix, pos) = create_matrix_and_start(input);
    let mut count = 0;
    
    let mut current_dir = 0;
    let mut current_pos = pos.clone();

    loop {
        let (new_pos, amount_moved, is_inside, _) = move_dir(&mut matrix, current_pos, DIRS[current_dir]);
        current_dir = (current_dir + 1) % DIRS.len();
        current_pos = new_pos;
        count += amount_moved;
        
        if !is_inside { 
            break
        }
    }
    
    Some((count) as u32)
}

fn contains_infinite_loops(matrix: &mut Vec<Vec<char>>, pos: (i32, i32), obs: (i32, i32)) -> bool {
    let mut clone = matrix.clone();
    clone[obs.0 as usize][obs.1 as usize] = '#';
    
    let mut current_dir = 0;
    let mut current_pos = pos.clone();
    
    let mut operations: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    loop {
        if operations.contains(&(current_pos, DIRS[current_dir])) { 
            return true;
        }
        
        operations.insert((current_pos, DIRS[current_dir]));
        let (new_pos, amount_moved, is_inside, _) = move_dir(&mut clone, current_pos, DIRS[current_dir]);
        current_dir = (current_dir + 1) % DIRS.len();
        current_pos = new_pos;
        
        if !is_inside { 
            return false;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut matrix, pos) = create_matrix_and_start(input);
    let mut possible_blocs: Vec<(i32, i32)> = Vec::new();
    
    let mut current_dir = 0;
    let mut current_pos = pos.clone();

    loop {
        let (new_pos, amount_moved, is_inside, new_bloc) = move_dir(&mut matrix, current_pos, DIRS[current_dir]);
        current_dir = (current_dir + 1) % DIRS.len();
        possible_blocs.extend(new_bloc);
        current_pos = new_pos;
        
        if !is_inside { 
            break
        }
    }
    
    let mut count = 0;
    for possible_bloc in possible_blocs.iter() {
        if contains_infinite_loops(&mut matrix, pos, *possible_bloc) {
            count += 1;
        }
    }
    Some(count)
}

fn create_matrix_and_start(input: &str) -> (Vec<Vec<char>>, (i32, i32)) {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut start = (0, 0);
    
    for (i, line) in input.lines().enumerate() {
        let row: Vec<char> = line.chars().collect();
        for (j, &c) in row.iter().enumerate() {
            if c == '^' { 
                start = (i as i32, j as i32);
                break;
            }
        }
        matrix.push(row);
    }
    
    (matrix, start)
}

fn move_dir(matrix: &mut Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32)) -> ((i32, i32), i32, bool, Vec<(i32, i32)>) {
    let mut move_count = 0;
    let mut new_pos = pos.clone();
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;
    
    let mut possible_obstacles: Vec<(i32, i32)> = Vec::new();

    loop {
        let old_pos = new_pos.clone();
        new_pos = (old_pos.0 + dir.0, old_pos.1 + dir.1);
        
        if !(new_pos.0 >= 0 && new_pos.0 < rows && new_pos.1 >= 0 && new_pos.1 < cols) { 
            return (new_pos, move_count, false, possible_obstacles);
        } else { 
            let value = matrix[new_pos.0 as usize][new_pos.1 as usize];
            if value == '#' {
                return (old_pos, move_count, true, possible_obstacles);
            } else if value != 'X' { 
                possible_obstacles.push(new_pos);
                move_count += 1;
                matrix[new_pos.0 as usize][new_pos.1 as usize] = 'X'
            }
        }
    }
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
