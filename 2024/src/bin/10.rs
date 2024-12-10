use std::collections::HashSet;

advent_of_code::solution!(10);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
}

struct Map(Vec<Vec<Option<u32>>>);

impl Map {
    fn new(input: &str) -> Self {
        let raw = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10)).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self(raw)
    }

    /// Finds neighbors that are inbounds.
    fn neighbors(&self, point: &Point) -> Vec<Point> {
        let mut results = vec![];
        if point.x > 0 {
            results.push(Point {
                x: point.x - 1,
                y: point.y,
            });
        }
        if (point.x as usize) < self.0.len() - 1 {
            results.push(Point {
                x: point.x + 1,
                y: point.y,
            });
        }
        if point.y > 0 {
            results.push(Point {
                x: point.x,
                y: point.y - 1,
            });
        }
        if (point.y as usize) < self.0.len() - 1 {
            results.push(Point {
                x: point.x,
                y: point.y + 1,
            });
        }
        results
    }

    /// Finds points on the map which have a height of zero.
    fn trailheads(&self) -> Vec<Point> {
        let mut trailheads = vec![];
        for (r, row) in self.0.iter().enumerate() {
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
        self.0
            .get(at.x as usize)
            .and_then(|row| row.get(at.y as usize))
            .and_then(|point| point.clone())
    }
}

/// A hiker is a person who follows the perfect hike strategy and explores
/// the map. It tracks score and rating, as well as remembers the peaks it
/// has reached. The implementation is identical for both part 1 and part 2.
#[derive(Clone)]
struct Hiker {
    score: u32,
    rating: u32,
    peaks_reached: HashSet<Point>,
}

impl Hiker {
    fn new() -> Self {
        Self {
            score: 0,
            rating: 0,
            peaks_reached: HashSet::new(),
        }
    }

    fn hike(&mut self, map: &Map, point: &Point) {
        let height = map.get_height(point).unwrap();
        if height == 9 {
            self.rating += 1;

            if !self.peaks_reached.contains(point) {
                self.peaks_reached.insert(point.clone());
                self.score += 1;
                return;
            }
        }
        for neighbor in map.neighbors(point).iter() {
            if let Some(neighbor_height) = map.get_height(&neighbor) {
                if neighbor_height == height + 1 {
                    self.hike(map, &neighbor);
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
        hiker.hike(&map, &trailhead);
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
        hiker.hike(&map, &trailhead);
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
