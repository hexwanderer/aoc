use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    fn subtract(one: &Self, other: &Self) -> Self {
        Self {
            x: one.x - other.x,
            y: one.y - other.y,
        }
    }

    fn add(one: &Self, distance: &Self) -> Self {
        Self {
            x: distance.x + one.x,
            y: distance.y + one.y,
        }
    }

    fn pythagorean(one: &Self, other: &Self) -> f32 {
        (((other.x - one.x).pow(2) + (other.y - one.y).pow(2)) as f32).sqrt()
    }
}

fn are_floats_equal(a: f32, b: f32, tolerance: f32) -> bool {
    (a - b).abs() < tolerance
}

struct Map {
    raw: HashMap<char, Vec<Point>>,
    max_size: Point,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut nodes = HashMap::<char, Vec<Point>>::new();
        let lines = input.lines();
        let rows = lines.collect::<Vec<_>>();
        let r = rows.len();
        let cols = rows[0].len();
        rows.iter().enumerate().for_each(|(r, line)| {
            line.chars().enumerate().for_each(|(c, val)| {
                if val == '.' {
                    return;
                }
                nodes
                    .entry(val)
                    .or_insert(Vec::new())
                    .push(Point::new(r, c));
            })
        });

        let size = Point {
            x: r as i32,
            y: cols as i32,
        };

        Map {
            raw: nodes,
            max_size: size,
        }
    }

    fn inbounds(&self, point: &Point) -> bool {
        return point.x >= 0
            && point.x < self.max_size.x
            && point.y >= 0
            && point.y < self.max_size.y;
    }

    fn inlines(&self, p1: &Point, p2: &Point, include_selves: bool) -> Vec<Point> {
        let distance = Point::subtract(&p2, &p1); // Calculate the distance once.
        let mut results = vec![];

        if include_selves {
            results.extend(vec![p1.clone(), p2.clone()]);
        }

        // Move backward from 'one' to 'other'.
        let mut point = Point::subtract(&p1, &distance);
        while self.inbounds(&point) {
            results.push(point.clone());
            point = Point::subtract(&point, &distance);
        }

        // Move forward from 'other' to 'one'.
        let mut point2 = Point::add(&p2, &distance);
        while self.inbounds(&point2) {
            results.push(point2.clone());
            point2 = Point::add(&point2, &distance);
        }

        results
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::new(input);

    let mut antinodes = HashSet::<Point>::new();
    map.raw.iter().for_each(|(_, points)| {
        for p1 in 0..points.len() {
            for p2 in p1 + 1..points.len() {
                let inlines = map.inlines(&points[p1], &points[p2], false);
                let local_antinodes = inlines
                    .into_iter()
                    .filter(|inline| {
                        let distance_to_p1 = Point::pythagorean(&points[p1], inline);
                        let distance_to_p2 = Point::pythagorean(&points[p2], inline);
                        if distance_to_p1 < distance_to_p2 {
                            are_floats_equal(distance_to_p1 * 2.0, distance_to_p2, 0.001)
                        } else {
                            are_floats_equal(distance_to_p2 * 2.0, distance_to_p1, 0.001)
                        }
                    })
                    .collect::<HashSet<_>>();
                antinodes = antinodes.union(&local_antinodes).cloned().collect();
            }
        }
    });

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::new(input);

    let mut antinodes = HashSet::<Point>::new();
    map.raw.iter().for_each(|(_, points)| {
        for p1 in 0..points.len() {
            for p2 in p1 + 1..points.len() {
                let inlines = map.inlines(&points[p1], &points[p2], true);
                let local_antinodes = inlines.into_iter().collect::<HashSet<_>>();
                antinodes = antinodes.union(&local_antinodes).cloned().collect();
            }
        }
    });

    Some(antinodes.len() as u32)
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
