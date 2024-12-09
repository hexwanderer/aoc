use std::collections::HashSet;

advent_of_code::solution!(6);

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

#[derive(Clone)]
struct Guard {
    position: (isize, isize),
    direction: Direction,
    visited: HashSet<(usize, usize)>,
}

impl Guard {
    fn next_move(&mut self, map: &mut Vec<Vec<Pieces>>, inbounds: &mut bool) {
        if let Pieces::Obstacle = map[self.position.0 as usize][self.position.1 as usize] {
            panic!("illegal position");
        }

        let mut pos: (isize, isize) = (0, 0);
        while !self.can_move_ahead(map, &mut pos, inbounds) {
            self.turn_ninety_degrees();
        }
        self.position = (pos.0, pos.1);
        if self.inbounds(map) {
            self.visited
                .insert((self.position.0 as usize, self.position.1 as usize));
        }
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

pub fn part_one(input: &str) -> Option<u32> {
    let mut guard = None;
    let mut map = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    '^' => {
                        guard = Some(Guard {
                            position: (r as isize, c as isize),
                            direction: Direction::North,
                            visited: HashSet::new(),
                        });
                        Pieces::Space
                    }
                    '.' => Pieces::Space,
                    '#' => Pieces::Obstacle,
                    _ => panic!("unknown object"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut guard = guard.unwrap();
    let mut inbounds = true;

    while inbounds {
        guard.next_move(&mut map, &mut inbounds);
    }

    Some(guard.visited.len() as u32)
}

#[derive(Clone)]
struct GuardWithLoopDetection {
    position: (isize, isize),
    direction: Direction,
    visited: HashSet<(usize, usize, Direction)>,
}

impl GuardWithLoopDetection {
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

pub fn part_two(input: &str) -> Option<u32> {
    let mut guard = None;
    let map = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    '^' => {
                        guard = Some(GuardWithLoopDetection {
                            position: (r as isize, c as isize),
                            direction: Direction::North,
                            visited: HashSet::new(),
                        });
                        Pieces::Space
                    }
                    '.' => Pieces::Space,
                    '#' => Pieces::Obstacle,
                    _ => panic!("unknown object"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let guard = guard.unwrap();
    let mut result = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == Pieces::Space
                && !(guard.position.0 as usize == i && guard.position.1 as usize == j)
            {
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
