use std::collections::HashSet;

advent_of_code::solution!(10);

#[derive(Hash, PartialEq, Eq, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
}

struct Map {
    raw: Vec<Vec<Option<u32>>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let raw = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10)).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { raw }
    }

    fn neighbors(&self, point: &Point) -> Vec<(Point, Direction)> {
        let mut results = vec![];
        if point.x > 0 {
            results.push((
                Point {
                    x: point.x - 1,
                    y: point.y,
                },
                Direction::West,
            ));
        }
        if (point.x as usize) < self.raw.len() - 1 {
            results.push((
                Point {
                    x: point.x + 1,
                    y: point.y,
                },
                Direction::East,
            ));
        }
        if point.y > 0 {
            results.push((
                Point {
                    x: point.x,
                    y: point.y - 1,
                },
                Direction::South,
            ));
        }
        if (point.y as usize) < self.raw.len() - 1 {
            results.push((
                Point {
                    x: point.x,
                    y: point.y + 1,
                },
                Direction::North,
            ));
        }
        results
    }

    fn trailheads(&self) -> Vec<Point> {
        let mut trailheads = vec![];
        for (r, row) in self.raw.iter().enumerate() {
            for (c, height) in row.iter().enumerate() {
                if let Some(height) = height {
                    if *height == 0 {
                        trailheads.push(Point {
                            x: r.clone() as u32,
                            y: c.clone() as u32,
                        });
                    }
                }
            }
        }
        trailheads
    }

    fn get_height(&self, at: &Point) -> Option<u32> {
        self.raw[at.x as usize][at.y as usize]
    }
}

#[derive(Clone)]
struct Hiker {
    score: u32,
    peaks_reached: HashSet<Point>,
    directions_used: HashSet<Vec<Direction>>,
    rating: u32,
}

impl Hiker {
    fn new() -> Self {
        Self {
            score: 0,
            peaks_reached: HashSet::new(),
            directions_used: HashSet::new(),
            rating: 0,
        }
    }

    fn hike(&mut self, map: &Map, point: &Point, direction_so_far: Vec<Direction>) {
        let height = map.get_height(point).unwrap();
        if height == 9 {
            if !self.directions_used.contains(&direction_so_far) {
                self.rating += 1;
                self.directions_used.insert(direction_so_far.clone());
            }

            if !self.peaks_reached.contains(point) {
                self.peaks_reached.insert(point.clone());
                self.score += 1;
                return;
            }
        }
        for (neighbor, direction) in map.neighbors(point).iter() {
            if let Some(neighbor_height) = map.get_height(&neighbor) {
                if neighbor_height == height + 1 {
                    let mut next_direction = direction_so_far.clone();
                    next_direction.push(direction.clone());
                    self.hike(map, &neighbor, next_direction);
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::new(input);
    let trailheads = map.trailheads();

    let mut hikers = vec![];
    for trailhead in trailheads {
        let mut hiker = Hiker::new();
        hiker.hike(&map, &trailhead, vec![]);
        hikers.push(hiker.clone());
    }

    hikers.iter().map(|h| h.score).reduce(|acc, v| acc + v)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::new(input);
    let trailheads = map.trailheads();

    let mut hikers = vec![];
    for trailhead in trailheads {
        let mut hiker = Hiker::new();
        hiker.hike(&map, &trailhead, vec![]);
        hikers.push(hiker.clone());
    }

    hikers.iter().map(|h| h.rating).reduce(|acc, v| acc + v)
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
