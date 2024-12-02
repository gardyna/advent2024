advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports: Vec<Vec<i32>> = into_reports(input);

    let num_safe = reports.iter().filter(is_safe).count();
    Some(num_safe as u32)
}

fn sign(n: &i32) -> i32 {
    if n == &0 {
        return 0;
    }
    n / n.abs()
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = into_reports(input);
    let mut safe_count = 0;
    
    for report in reports {
        if is_safe(&&report) { 
            safe_count += 1;
        } else {
            for i in 0..report.len() {
                let mut modified_report = Vec::from(&report[..i]);
                modified_report.extend_from_slice(&report[i + 1..]);
                if is_safe(&&modified_report) { 
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    Some(safe_count as u32)
}

fn is_safe(report: &&Vec<i32>) -> bool {
    let mut diff_iter = report.iter().zip(report.iter().skip(1)).map(|(x, y)| x - y);
    let diff = diff_iter.next().unwrap();
    if diff == 0 || diff.abs() > 3 {
        return false;
    }
    let s = sign(&diff);
    diff_iter.all(|d| s * d > 0 && d.abs() < 4)
}

fn into_reports(input: &str) -> Vec<Vec<i32>> {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect();
    reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
