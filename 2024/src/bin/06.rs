use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Clone)]
struct Guard {
    position: (isize, isize),
    direction: Direction,
    visited: HashSet<(usize, usize, Direction)>,
}

impl Guard {
    fn next_move(&mut self, map: &mut Vec<Vec<Pieces>>, inbounds: &mut bool) -> bool {
        if let Pieces::Obstacle = map[self.position.0 as usize][self.position.1 as usize] {
            panic!("illegal position");
        }

        let mut pos: (isize, isize) = (0, 0);
        while !self.can_move_ahead(map, &mut pos, inbounds) {
            self.turn_ninety_degrees();
        }
        self.position = (pos.0, pos.1);
        if self.inbounds(map) {
            if self.visited.contains(&(
                self.position.0 as usize,
                self.position.1 as usize,
                self.direction,
            )) {
                return true;
            }
            self.visited.insert((
                self.position.0 as usize,
                self.position.1 as usize,
                self.direction,
            ));
        }
        false
    }

    fn turn_ninety_degrees(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        };
    }

    fn can_move_ahead(
        &self,
        map: &Vec<Vec<Pieces>>,
        next_pos: &mut (isize, isize),
        inbounds: &mut bool,
    ) -> bool {
        let next_position = match self.direction {
            Direction::North => (self.position.0 - 1, self.position.1),
            Direction::South => (self.position.0 + 1, self.position.1),
            Direction::West => (self.position.0, self.position.1 - 1),
            Direction::East => (self.position.0, self.position.1 + 1),
        };
        *next_pos = next_position;
        let is_in = Self::inbounds_for_position(map, next_position);
        *inbounds = is_in;
        if is_in {
            Pieces::Obstacle != map[next_position.0 as usize][next_position.1 as usize]
        } else {
            true
        }
    }

    fn inbounds(&self, map: &Vec<Vec<Pieces>>) -> bool {
        Self::inbounds_for_position(map, self.position)
    }

    fn inbounds_for_position(map: &Vec<Vec<Pieces>>, position: (isize, isize)) -> bool {
        ((0 as isize)..(map.len() as isize)).contains(&position.0)
            && ((0 as isize)..(map[0].len() as isize)).contains(&position.1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pieces {
    Space,
    Obstacle,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn new(c: char) -> Option<Self> {
        match c {
            '<' => Some(Direction::West),
            '>' => Some(Direction::East),
            '^' => Some(Direction::North),
            'v' => Some(Direction::South),
            _ => None,
        }
    }
}

fn build_map(input: &str) -> (Vec<Vec<Pieces>>, (isize, isize), Direction) {
    let mut position = None;
    let mut direction = None;
    let map = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    '^' | '<' | '>' | 'v' => {
                        position = Some((r as isize, c as isize));
                        direction = Direction::new(ch);
                        Pieces::Space
                    }
                    '.' => Pieces::Space,
                    '#' => Pieces::Obstacle,
                    _ => panic!("unknown object"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Each valid problem must have a starting guard position and direction
    (map, position.unwrap(), direction.unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, position, direction) = build_map(input);

    let mut guard = Guard {
        position,
        direction,
        visited: HashSet::new(),
    };

    // We know the guard is initially inbounds
    let mut inbounds = true;

    // Use the mutable variable to track inbounds
    while inbounds {
        guard.next_move(&mut map, &mut inbounds);
    }

    Some(
        // The guard's visited property tracks each unique (row, column, direction) set,
        // for reasons we'll see in part 2.
        // So build a new set of visited based on (row, column) pairs only
        guard
            .visited
            .iter()
            .map(|(x, y, _)| (x, y))
            .collect::<HashSet<_>>()
            .len() as u32,
    )
}

/// We will need the direction later, but for now, we just need to get
/// every position that the guard can reach.
///
/// We use the direction to track if we are in a loop. We can visit the
/// same location multiple times, but if we are facing different directions
/// it doesn't mean much. If we come in at the same direction though, we
/// know this is a loop, which is why we track direction.
///
/// Finally, while a naive implementation of this problem solves by brute
/// forcing every possible space, we can observe that the only valid answers
/// must be within the path the guard visits on a 'normal' path. Hence, we
/// prune down the searchable space to just the reachable path.
pub fn part_two(input: &str) -> Option<u32> {
    let (mut map, position, direction) = build_map(input);

    let guard = Guard {
        position,
        direction,
        visited: HashSet::new(),
    };

    let mut guard_copy = guard.clone();

    let mut inbounds = true;
    while inbounds {
        guard_copy.next_move(&mut map, &mut inbounds);
    }

    let possible_paths = guard_copy
        .visited
        .iter()
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<_>>();

    let mut result = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let is_empty_space = map[i][j] == Pieces::Space;
            let is_guards_spot = guard.position.0 as usize == i && guard.position.1 as usize == j;
            let is_reachable = possible_paths.contains(&(&i, &j));
            if is_empty_space && !is_guards_spot && is_reachable {
                let mut local_guard = guard.clone();
                let mut inbounds = true;
                let mut deep_copy: Vec<Vec<Pieces>> = map.iter().map(|v| (*v).clone()).collect();
                deep_copy[i][j] = Pieces::Obstacle;

                while inbounds {
                    let loops = local_guard.next_move(&mut deep_copy, &mut inbounds);
                    if loops {
                        result += 1;
                        break;
                    }
                }
            }
        }
    }

    Some(result)
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
