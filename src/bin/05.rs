use std::cmp::Ordering;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = extract_rules_and_updates(input);
    let result: u32 = updates.into_iter()
        .filter(|pages| {
            pages.is_sorted_by(|a, b| !rules.contains(&(*b, *a)))
        })
        .map(|pages| pages[pages.len() / 2])
        .sum();
    Some(result)
}



pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = extract_rules_and_updates(input);
    
    let result: u32 = updates.into_iter()
        .filter(|pages| {
            !pages.is_sorted_by(|a, b| !rules.contains(&(*b, *a)))
        })
        .map(|mut update| {
            update.sort_by(|a, b| {
                if rules.contains(&(*a, *b)) { 
                    Ordering::Greater
                } else if rules.contains(&(*b, *a)) { 
                    Ordering::Less
                } else { 
                    Ordering::Equal
                }
            });
            update
        })
        .map(|pages| pages[pages.len() / 2])
        .sum();
    Some(result)
}

fn extract_rules_and_updates(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules.lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    let updates = updates.lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
