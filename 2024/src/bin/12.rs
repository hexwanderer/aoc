use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

advent_of_code::solution!(12);

#[derive(Clone, PartialEq, Eq, Debug)]
struct Point(i32, i32);

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

const DIR: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

impl Point {
    fn perimeter(points: &HashSet<&Point>) -> u32 {
        let mut perimeter = 0;
        for point in points {
            let mut local_perimeter = 4;
            if points.contains(&Point(point.0 + 1, point.1)) {
                local_perimeter -= 1;
            }
            if points.contains(&Point(point.0 - 1, point.1)) {
                local_perimeter -= 1;
            }
            if points.contains(&Point(point.0, point.1 + 1)) {
                local_perimeter -= 1;
            }
            if points.contains(&Point(point.0, point.1 - 1)) {
                local_perimeter -= 1;
            }
            perimeter += local_perimeter
        }
        perimeter
    }

    fn sides(points: &HashSet<&Point>) -> u32 {
        let mut side_count: u32 = 0;
        for dir in DIR {
            let mut sides = HashSet::new();
            for point in points {
                let tmp = Point(point.0 + dir.0, point.1 + dir.1);
                if !points.contains(&tmp) {
                    sides.insert(tmp);
                }
            }
            let mut remove = HashSet::new();
            for side in sides.clone() {
                let mut tmp = Point(side.0 + dir.1, side.1 + dir.0);
                while sides.contains(&tmp) {
                    remove.insert(tmp.clone());
                    tmp = Point(tmp.0 + dir.1, tmp.1 + dir.0);
                }
            }
            side_count += (sides.len() - remove.len()) as u32
        }
        side_count
    }
}

#[derive(Debug)]
struct UnionFind(HashMap<Point, Point>);

impl UnionFind {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, x: Point) {
        self.0.entry(x.clone()).or_insert(x.clone());
    }

    fn find(&mut self, x: &Point) -> Point {
        if let Some(parent) = self.0.get(x).cloned() {
            if parent != *x {
                // Path compression
                let root = self.find(&parent);
                self.0.insert(x.clone(), root.clone());
                root
            } else {
                parent
            }
        } else {
            // If there's no parent, the point is its own parent
            self.0.insert(x.clone(), x.clone());
            x.clone()
        }
    }

    fn union(&mut self, x: &Point, y: &Point) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx != ry {
            self.0.insert(ry, rx);
        }
    }
}

fn process(input: &str) -> (UnionFind, HashMap<Point, Vec<Point>>) {
    let mut union_find = UnionFind::new();
    let mut points = vec![];
    let map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .map(|(c, val)| {
                    let p = Point(r as i32, c as i32);
                    points.push(p.clone());
                    union_find.insert(p.clone());
                    val
                })
                .collect()
        })
        .collect();

    for (r, row) in input.lines().enumerate() {
        for (c, _) in row.chars().enumerate() {
            if r >= 1 {
                if map[r][c] == map[r - 1][c] {
                    let p1 = Point(r as i32, c as i32);
                    let p2 = Point((r - 1) as i32, c as i32);
                    union_find.union(&p1, &p2);
                }
            }
            if c >= 1 {
                if map[r][c] == map[r][c - 1] {
                    let p1 = Point(r as i32, c as i32);
                    let p2 = Point(r as i32, (c - 1) as i32);
                    union_find.union(&p1, &p2);
                }
            }
        }
    }

    let mut reverse_map = HashMap::<Point, Vec<Point>>::new();
    for point in points {
        let parent = union_find.find(&point);
        reverse_map.entry(parent).or_insert(Vec::new()).push(point);
    }
    (union_find, reverse_map)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, reverse_map) = process(input);

    reverse_map
        .iter()
        .map(|(_, points)| {
            let area = points.len() as u32;
            let item_set = points.iter().collect::<HashSet<_>>();
            let perimeter = Point::perimeter(&item_set);
            area * perimeter
        })
        .reduce(|acc, v| acc + v)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, reverse_map) = process(input);

    reverse_map
        .iter()
        .map(|(_, points)| {
            let area = points.len() as u32;
            let item_set = points.iter().collect::<HashSet<_>>();
            let sides = Point::sides(&item_set);
            area * sides
        })
        .reduce(|acc, v| acc + v)
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
