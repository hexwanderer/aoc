use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn subtract(one: &Self, other: &Self) -> Self {
        Self {
            x: other.x - one.x,
            y: other.y - one.y,
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

    fn inbounds(&self, map_size: &Self) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < map_size.x && self.y < map_size.y
    }

    fn inlines(one: &Self, other: &Self, map_size: &Self) -> Vec<Point> {
        let distance = Self::subtract(one, other); // Calculate the distance once.
        let mut results = vec![];

        // Move backward from 'one' to 'other'.
        let point = Self::subtract(one, &distance);
        if point.inbounds(&map_size) {
            results.push(point.clone());
        }

        // Move forward from 'other' to 'one'.
        let point2 = Self::add(other, &distance);
        if point2.inbounds(&map_size) {
            results.push(point2.clone());
        }
        results
    }
}

fn are_floats_equal(a: f32, b: f32, tolerance: f32) -> bool {
    (a - b).abs() < tolerance
}

pub fn part_one(input: &str) -> Option<u32> {
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
            nodes.entry(val).or_insert(Vec::new()).push(Point {
                x: r as i32,
                y: c as i32,
            });
        })
    });

    let size = Point {
        x: r as i32,
        y: cols as i32,
    };

    let mut antinodes = HashSet::<Point>::new();
    nodes.iter().for_each(|(_channel, points)| {
        for p1 in 0..points.len() {
            for p2 in p1 + 1..points.len() {
                let inlines = Point::inlines(&points[p1], &points[p2], &size);
                println!(
                    "p1 {:?}, p2: {:?}, inlines: {:?}",
                    points[p1], points[p2], inlines
                );
                let local_antinodes = inlines
                    .into_iter()
                    .filter(|inline| {
                        let distance_to_p1 = Point::pythagorean(&points[p1], inline);
                        let distance_to_p2 = Point::pythagorean(&points[p2], inline);
                        if distance_to_p1 > distance_to_p2 {
                            are_floats_equal(distance_to_p1 * 2.0, distance_to_p2, 0.001)
                        } else {
                            are_floats_equal(distance_to_p2 * 2.0, distance_to_p1, 0.001)
                        }
                    })
                    .collect::<HashSet<_>>();
                println!(
                    "antinodes for {:?} {:?}: {:?}",
                    &points[p1], &points[p2], &local_antinodes
                );
                antinodes = antinodes.union(&local_antinodes).cloned().collect();
            }
        }
    });

    Some(antinodes.len() as u32)
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
