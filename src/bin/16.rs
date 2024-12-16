use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

advent_of_code::solution!(16);

const TURN_COST: usize = 1_000;
const MOVE_COST: usize = 1;

type Position = (i32, i32);
type Direction = (i32, i32);

#[derive(Debug)]
struct RaceSetup {
    obstacles: HashSet<Position>,
    start: Position,
    start_direction: Direction,
    end: Position,
}

fn parse(input: &str) -> RaceSetup {
    let mut start = (0, 0);
    let start_direction = (1, 0);
    let mut end = (0, 0);
    
    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
    
    input.lines().enumerate().for_each(|(y, row)| {
       row.chars().enumerate().for_each(|(x, tile)| match tile {
           '#' => {
               obstacles.insert((x as i32, y as i32));
           }
           'S' => {
               start = (x as i32, y as i32);
           }
           'E' => {
               end = (x as i32, y as i32);
           }
           _ => () 
       })
    });
    
    RaceSetup {
        obstacles,
        start,
        start_direction,
        end
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: Position,
    direction: Direction,
    end: Position,
    score: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
            .then_with(|| {
                ((self.position.0 - self.end.0).abs() + (self.position.1 - self.end.1).abs()).cmp(
                    &((other.position.0 - other.end.0).abs() 
                            + (other.position.1 - other.end.1).abs()),
                )
            })
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let setup = parse(input);
    let mut min_scores: HashMap<(Position, Direction), usize> = HashMap::new();
    let mut frontier = BinaryHeap::new();
    
    frontier.push(State {
        position: setup.start,
        direction: setup.start_direction,
        end: setup.end,
        score: 0
    });
    
    while let Some(State {
        position, direction, end, score
                   }) = frontier.pop() {
        if position == end {
            return Some(score);
        }
        
        let min_score = min_scores.entry((position, direction)).or_insert(score);
        
        if score > *min_score {
            continue;
        }
        
        for new_state in [
            State {
                position: (position.0 + direction.0, position.1 + direction.1),
                direction,
                end,
                score: score + MOVE_COST,
            },
            State {
                position,
                direction: (direction.1, direction.0),
                end,
                score: score + TURN_COST,
            },
            State {
                position,
                direction: (-direction.1, -direction.0),
                end,
                score: score + TURN_COST
            }
        ] {
            if setup.obstacles.contains(&new_state.position) { 
                continue;
            }
            
            let min_score = min_scores
                .entry((new_state.position, new_state.direction))
                .or_insert(new_state.score + 1);
            
            if new_state.score < *min_score {
                *min_score = new_state.score;
                frontier.push(new_state);
            }
        }
    }
    None
}

#[derive(Clone, Eq, PartialEq)]
struct ExtendedState {
    position: Position,
    direction: Direction,
    end: Position,
    score: usize,
    path: Vec<Position>,
}

impl Ord for ExtendedState {
    fn cmp(&self, other: &Self) -> Ordering {
        State {
            position: self.position,
            direction: self.direction,
            end: self.end,
            score: self.score
        }.cmp(&State {
            position: other.position,
            direction: other.direction,
            end: other.end,
            score: other.score
        })
    }
}

impl PartialOrd for ExtendedState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let setup = parse(input);
    let mut min_scores: HashMap<(Position, Direction), usize> = HashMap::new();
    let mut final_score: Option<usize> = None;
    let mut best_seats: HashSet<Position> = HashSet::new();
    
    let mut frontier = BinaryHeap::new();
    frontier.push(ExtendedState {
        position: setup.start,
        direction: setup.start_direction,
        end: setup.end,
        score: 0,
        path: vec![setup.start],
    });
    
    while let Some(ExtendedState {
        position, direction, end, score, path
                   }) = frontier.pop() {
        match final_score {
            Some(old_score) if old_score < score => {
                continue
            }
            _ => {
                if position == end { 
                    final_score = Some(score);
                    best_seats.extend(path.into_iter());
                    continue
                }
            }
        };
        
        let min_score = min_scores.entry((position, direction)).or_insert(score);
        
        if score > *min_score {
            continue;
        }

        for new_state in [
            ExtendedState {
                position: (position.0 + direction.0, position.1 + direction.1),
                direction,
                end,
                score: score + MOVE_COST,
                path: {
                    let mut new_path = path.clone();
                    new_path.push((position.0 + direction.0, position.1 + direction.1));
                    new_path
                }
            },
            ExtendedState {
                position,
                direction: (direction.1, direction.0),
                end,
                score: score + TURN_COST,
                path: path.clone()
            },
            ExtendedState {
                position,
                direction: (-direction.1, -direction.0),
                end,
                score: score + TURN_COST,
                path: path.clone(),
            },
        ] {
            if setup.obstacles.contains(&new_state.position) {
                continue;
            }
            let min_score = min_scores.entry((new_state.position, new_state.direction))
                .or_insert(new_state.score + 1);
            
            if new_state.score <= *min_score {
                *min_score = new_state.score;
                frontier.push(new_state)
            }
        }
    }
    Some(best_seats.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
