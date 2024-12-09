advent_of_code::solution!(7);

fn is_possible(result: u64, values: &[u64]) -> bool {
    let Some(&top) = values.first() else {
        return false;
    };
    (values.len() == 1 && result == top)
        || (result % top == 0 && is_possible(result / top, &values[1..]))
        || (result > top && is_possible(result - top, &values[1..]))
}

fn is_possible_concat(result: u64, values: &[u64]) -> bool {
    let Some(&top) = values.first() else {
        return false;
    };
    (values.len() == 1 && result == top)
    || (result % top == 0 && is_possible_concat(result / top, &values[1..]))
    || (result > top && is_possible_concat(result - top, &values[1..]))
    || result
        .checked_sub(top)
        .zip(top.checked_ilog10().map(|log10| 10u64.pow(log10 + 1)))
        .and_then(|(delta, power_of_10)| {
            (delta % power_of_10 == 0).then_some(is_possible_concat(delta / power_of_10, &values[1..]))
        }).unwrap_or_default()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let (target, nums) = line.split_once(":").expect("bad line!");
        let target = target.trim().parse().unwrap();
        let nums = nums.trim().split(' ')
            .map(|x| x.trim().parse())
            .rev().collect::<Result<Vec<u64>, _>>().unwrap();
        if !is_possible(target, &nums) {
            continue;
        }
        sum += target;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let (target, nums) = line.split_once(":").expect("bad line!");
        let target = target.trim().parse().unwrap();
        let nums = nums.trim().split(' ')
            .map(|x| x.trim().parse())
            .rev().collect::<Result<Vec<u64>, _>>().unwrap();
        if !is_possible_concat(target, &nums) {
            continue;
        }
        sum += target;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("156: 15 6");
        assert_eq!(result, Some(156));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
