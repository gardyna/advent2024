use regex::Regex;
use advent_of_code::is_integer;

advent_of_code::solution!(13);

const CORRECTION: f64 = 10000000000000.0;


#[derive(Debug)]
struct Arcade {
    button_a_x: i32,
    button_a_y: i32,
    button_b_x: i32,
    button_b_y: i32,
    price_x: i32,
    price_y: i32,
}

impl Arcade {
    fn solve(&self, part: u8) -> Option<[f64; 2]> {
        let a = self.button_a_x as f64;
        let b = self.button_b_x as f64;
        let c = self.button_a_y as f64;
        let d = self.button_b_y as f64;

        let det = a * d - b * c;

        if det == 0.0 {
            return None;
        }
        let price_vec: [f64; 2];

        if part == 1 {
            price_vec = [self.price_x as f64, self.price_y as f64];
        } else {
            price_vec = [self.price_x as f64 + CORRECTION, self.price_y as f64 + CORRECTION];
        }
        let press_a = (d * price_vec[0] - b * price_vec[1]) / det;
        let press_b  = (-c * price_vec[0] + a * price_vec[1]) / det;

        Some([press_a, press_b])
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();

    let mut score = 0;
    
    for cap in re.captures_iter(input) {
        let arcade = Arcade {
            button_a_x: cap[1].parse().unwrap(),
            button_a_y: cap[2].parse().unwrap(),
            button_b_x: cap[3].parse().unwrap(),
            button_b_y: cap[4].parse().unwrap(),
            price_x: cap[5].parse().unwrap(),
            price_y: cap[6].parse().unwrap()
        };

        if let Some(solution) = arcade.solve(1) {
            let a = solution[0];
            let b = solution[1];

            if is_integer(a) && is_integer(b) {
                score += a as u32 * 3 + b as u32;
            }
        }
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<i64> {

    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();

    let mut score = 0;

    for cap in re.captures_iter(input) {
        let arcade = Arcade {
            button_a_x: cap[1].parse().unwrap(),
            button_a_y: cap[2].parse().unwrap(),
            button_b_x: cap[3].parse().unwrap(),
            button_b_y: cap[4].parse().unwrap(),
            price_x: cap[5].parse().unwrap(),
            price_y: cap[6].parse().unwrap()
        };

        if let Some(solution) = arcade.solve(2) {
            let a = solution[0];
            let b = solution[1];

            if is_integer(a) && is_integer(b) {
                score += a as i64 * 3 + b as i64;
            }
        }
    }
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
