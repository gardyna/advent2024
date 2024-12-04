advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        matrix.push(line.chars().collect());
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let cur_char = matrix[i][j];

            if cur_char != 'X' { continue; }

            // is Xmas horizontal?
            if matrix[i].len() - j >= 4 {
                let horizontal: String = matrix[i][j..(j + 4)].iter().collect();
                if horizontal == "XMAS" {
                    sum += 1;
                }
            }

            if j >= 3 {
                let mut slice = matrix[i][(j - 3)..=j].to_vec();
                slice.reverse();
                let horizontal_reverse: String = slice.iter().collect();
                if horizontal_reverse == "XMAS" {
                    sum += 1;
                }
            }

            if matrix.len() - i >= 4 {
                let vertical: String = matrix[i..(i + 4)].iter().map(|inner| inner[j]).collect();
                if vertical == "XMAS" {
                    sum += 1;
                }
            }

            if i >= 3 {
                let mut slice = matrix[(i - 3)..=i].to_vec();
                slice.reverse();
                let vertical_reverse: String = slice.iter().map(|inner| inner[j]).collect();
                if vertical_reverse == "XMAS" {
                    sum += 1;
                }
            }

            if matrix.len() - i >= 4 && matrix[i].len() - j >= 4 {
                let mut diagonal_bottom_right = String::new();
                let slice = &matrix[i..(i + 4)];

                for k in 0..4 {
                    diagonal_bottom_right.push(slice[k][j + k]);
                }

                if diagonal_bottom_right == "XMAS" {
                    sum += 1;
                }
            }

            if matrix.len() - i >= 4 && j >= 3 {
                let mut diagonal_bottom_left = String::new();
                let slice = &matrix[i..(i + 4)];

                for k in 0..4 {
                    diagonal_bottom_left.push(slice[k][j - k]);
                }

                if diagonal_bottom_left == "XMAS" {
                    sum += 1;
                }
            }

            if i >= 3 && j >= 3 {
                let mut diagonal_top_left = String::new();
                let mut slice = matrix[(i - 3)..=i].to_vec();
                slice.reverse();

                for k in 0..4 {
                    diagonal_top_left.push(slice[k][j - k]);
                }

                if diagonal_top_left == "XMAS" {
                    sum += 1;
                }
            }

            if i >= 3 && matrix[i].len() - j >= 4 {
                let mut diagonal_top_right = String::new();
                let mut slice = matrix[(i - 3)..=i].to_vec();
                slice.reverse();

                for k in 0..4 {
                    diagonal_top_right.push(slice[k][j + k]);
                }

                if diagonal_top_right == "XMAS" {
                    sum += 1;
                }
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        matrix.push(line.chars().collect());
    }
    
    for i in 1..matrix.len() {
        for j in 1..matrix[i].len() {
            let cur_char = matrix[i][j];
            if cur_char != 'A' { continue; }
            
            let mut found_left = false;
            let mut found_right = false;
            
            if matrix.len() - i >= 2 && matrix[i].len() -j >= 2 { 
                let mut diagonal_left = String::new();
                let mut diagonal_right = String::new();
                
                let slice = &matrix[(i - 1)..=(i + 1)];
                
                for k in 0..3 {
                    diagonal_left.push(slice[k][j - 1 + k]);
                }
                
                if diagonal_left == "MAS" || diagonal_left == "SAM" { 
                    found_left = true;
                }
                let mut slice = slice.to_vec();
                slice.reverse();

                for k in 0..3 {
                    diagonal_right.push(slice[k][j - 1 + k]);
                }
                
                if diagonal_right == "MAS" || diagonal_right == "SAM" {
                    found_right = true;
                }
            }
            if found_right && found_left { sum += 1; }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
