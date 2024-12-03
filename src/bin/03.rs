use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // capture all mul commands
    let instuctions = regex_extract(input, r"mul\(\d{1,4},\d{1,4}\)");
    let vals: Vec<u32> = instuctions.into_iter()
        .map(|v| regex_extract(&v, r"\d{1,4}").into_iter()
            .map(|w| w.parse::<u32>().unwrap()).collect()).collect::<Vec<Vec<u32>>>().into_iter()
        .map(|x: Vec<u32>| x[0] * x[1]).collect();
    Some(vals.iter().sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = regex_extract(input, r"mul\(\d{1,4}\,\d{1,4}\)|don?\'?t?\(\)");
    let mut should_run = true;
    let mut collector = 0;

    for instr in instructions {
        match instr.as_str() {
            "do()" => should_run = true,
            "don't()" => should_run = false,
            _ => {
                if should_run {
                    collector += regex_extract(instr.as_str(), r"\d{1,4}").into_iter()
                        .map(|w| w.parse().unwrap()).collect::<Vec<u32>>().into_iter().product::<u32>();
                }
            }
        }
    }
    Some(collector)
}

fn regex_extract(string: &str, pattern: &str) -> Vec<String> {
    let re = Regex::new(pattern).unwrap();
    let matches = re.find_iter(string);
    matches.map(|v| v.as_str().to_owned()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }
}
