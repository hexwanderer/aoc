advent_of_code::solution!(6);

#[derive(Debug)]
enum Pieces {
    Space(bool),
    Obstacle(bool),
}

impl Pieces {
    fn visit(&mut self) {
        match self {
            Pieces::Space(ref mut visited) | Pieces::Obstacle(ref mut visited) => {
                *visited = true;
            }
        }
    }
}

fn get_elem<T>(vector: &Vec<T>, index: isize) -> Option<&T> {
    if index < 0 {
        None
    } else {
        vector.get(index as usize)
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

struct Guard {
    position: (isize, isize),
    direction: Direction,
}

impl Guard {
    fn next_move(&mut self, map: &mut Vec<Vec<Pieces>>) {
        if let Pieces::Obstacle(_) = map[self.position.0 as usize][self.position.1 as usize] {
            panic!("illegal position");
        }

        let mut pos: (isize, isize) = (0, 0);
        while !self.can_move_ahead(map, &mut pos) {
            self.turn_ninety_degrees();
        }
        self.position = (pos.0, pos.1);
        // Use custom `get_elem` function while avoiding conflicting borrows
        let row = get_elem(map, self.position.0);
        if let Some(row) = row {
            if let Some(val) = get_elem(row, self.position.1) {
                // Temporarily release the mutable borrow of `map` for safe access
                let val_mut = val as *const Pieces as *mut Pieces; // Cast to raw pointer
                unsafe {
                    (*val_mut).visit();
                } // Access mutable reference using raw pointer
            }
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

    fn can_move_ahead(&self, map: &Vec<Vec<Pieces>>, next_pos: &mut (isize, isize)) -> bool {
        let next_position = match self.direction {
            Direction::North => (self.position.0 - 1, self.position.1),
            Direction::South => (self.position.0 + 1, self.position.1),
            Direction::West => (self.position.0, self.position.1 - 1),
            Direction::East => (self.position.0, self.position.1 + 1),
        };
        *next_pos = next_position;
        match if let Some(row) = get_elem(map, next_position.0) {
            get_elem(row, next_position.1)
        } else {
            None
        } {
            Some(Pieces::Obstacle(_)) => false,
            _ => true,
        }
    }

    fn inbounds(&self, map: &Vec<Vec<Pieces>>) -> bool {
        ((0 as isize)..(map.len() as isize)).contains(&self.position.0)
            && ((0 as isize)..(map[0].len() as isize)).contains(&self.position.1)
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
                        });
                        Pieces::Space(true)
                    }
                    '.' => Pieces::Space(false),
                    '#' => Pieces::Obstacle(false),
                    _ => panic!("unknown object"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut guard = guard.unwrap();

    while guard.inbounds(&map) {
        guard.next_move(&mut map);
    }

    map.iter()
        .map(|row| {
            row.iter()
                .filter(|elem| match elem {
                    Pieces::Space(visited) => *visited,
                    Pieces::Obstacle(visited) => *visited,
                })
                .count() as u32
        })
        .reduce(|acc, v| acc + v)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
