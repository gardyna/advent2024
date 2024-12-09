use std::collections::{BTreeSet, HashMap};

advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut parsed: Vec<(usize, usize)> = Vec::new();
    let mut blk = 0;
    let mut iter = input.chars().enumerate();
    loop {
        match iter.next() {
            Some((i, c)) => {
                let val = c.to_digit(10).unwrap() as usize;
                if (i % 2) == 0 { 
                    blk = val;
                } else { 
                    parsed.push((blk, val));
                }
            }
            None => {
                parsed.push((blk, 0));
                break;
            }
        }
    }
    parsed
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);
    let mut res: Vec<usize> = Vec::new();
    let mut j = input.len() - 1;
    let (mut curr_tail_blk, _curr_tail_free) = input[j];

    for (i, (blk, mut free)) in input.iter().enumerate() {
        if i == j { 
            if curr_tail_blk > 0 { 
                res.extend(vec![j; curr_tail_blk]);
            }
            break;
        }
        let curr_blk = vec![i; *blk as usize];
        res.extend(curr_blk);
        while free > 0 {
            if free >= curr_tail_blk { 
                free = free - curr_tail_blk;
                res.extend(vec![j; curr_tail_blk]);
                j -= 1;
                curr_tail_blk = input[j].0;
            } else { 
                curr_tail_blk -= free;
                res.extend(vec![j; free]);
                free = 0;
            }
        }
    }
    Some(res.iter().enumerate().map(|(i, v)| (i as u64) * (*v as u64)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let mut block_pos: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut free_block:BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut curr_pos = 0;
    for (i, (blk_size, free)) in input.iter().enumerate() {
        block_pos.insert(i, (curr_pos, curr_pos + blk_size));
        curr_pos += blk_size;
        if free > &0 { 
            free_block.insert((curr_pos, curr_pos + free));
            curr_pos += free;
        }
    }
    
    let mut j = input.len() - 1;
    while j > 0 {
        let (start_pos, end_pos) = block_pos.get(&j).unwrap();
        let blk_len = end_pos - start_pos;
        let mut v_opt = None;
        for (x, y) in free_block.iter() {
            if x > start_pos { 
                break;
            }
            if (y - x) >= blk_len { 
                v_opt = Some((*x, *y));
                break;
            }
        }
        if let Some(v) = v_opt {
            let new_end = v.0 + blk_len;
            block_pos.insert(j, (v.0, new_end));
            free_block.remove(&v);
            if new_end < v.1 { 
                free_block.insert((new_end, v.1));
            }
        }
        j -= 1;
    }
    Some(block_pos.into_iter()
        .map(|(k, (start, end))|{
            let mut acc = 0;
            for i in start..end {
                acc += k * i;
            }
            acc
        }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
