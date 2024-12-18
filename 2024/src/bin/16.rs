use std::collections::{BinaryHeap, HashMap, HashSet};

advent_of_code::solution!(16);

#[derive(Debug, PartialEq, Eq)]
enum Elem {
    Wall,
    Empty,
}

type Maze = Vec<Vec<Elem>>;

fn process(input: &str) -> (Maze, (usize, usize), (usize, usize)) {
    let (mut start, mut end) = (None, None);
    let map = input
        .lines()
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    '#' => Elem::Wall,
                    '.' => Elem::Empty,
                    'S' => {
                        start = Some((r, c));
                        Elem::Empty
                    }
                    'E' => {
                        end = Some((r, c));
                        Elem::Empty
                    }
                    _ => panic!("unreachable"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (map, start.unwrap(), end.unwrap())
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
    direction: usize, // direction: 0 -> East, 1 -> South, 2 -> West, 3 -> North
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // reverse ordering for a min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start, goal) = process(input);
    let mut visited = HashSet::new();
    let mut dist = HashMap::new();
    let mut todo = BinaryHeap::new();

    // Start state facing East (direction = 0)
    dist.insert(start, 0);
    todo.push(State {
        cost: 0,
        position: start,
        direction: 0, // East
    });

    // Directions encoded as: 0 -> right (East), 1 -> down (South), 2 -> left (West), 3 -> up (North)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(State {
        cost,
        position,
        direction,
    }) = todo.pop()
    {
        if position == goal {
            return Some(cost); // Found the shortest path to the goal
        }

        if visited.contains(&(position, direction)) {
            continue;
        }

        visited.insert((position, direction));

        for (new_direction, &(dx, dy)) in directions.iter().enumerate() {
            let (new_r, new_c) = (position.0 as isize + dx, position.1 as isize + dy);
            if new_r < 0 || new_c < 0 {
                continue;
            }

            let new_pos = (new_r as usize, new_c as usize);
            if new_pos.0 >= map.len() || new_pos.1 >= map[0].len() {
                continue;
            }

            if map[new_pos.0][new_pos.1] == Elem::Wall {
                continue;
            }

            // Calculate additional cost if direction changes
            let mut new_cost = cost + 1; // normal move cost
            if direction != new_direction {
                new_cost += 1000; // 90-degree turn costs 1000
            }

            if new_cost < *dist.entry(new_pos).or_insert(u32::MAX) {
                dist.insert(new_pos, new_cost);
                todo.push(State {
                    cost: new_cost,
                    position: new_pos,
                    direction: new_direction, // Update direction
                });
            }
        }
    }

    None // If no path is found
}

#[derive(Clone, Eq, PartialEq)]
struct P2State {
    cost: u32,
    position: (usize, usize),
    direction: usize,
    visited: HashSet<(usize, usize)>,
}

impl Ord for P2State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for P2State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start, goal) = process(input);
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut dist: HashMap<((usize, usize), usize), u32> = HashMap::new();
    let mut pq = BinaryHeap::new();
    dist.insert((start, 0), 0);
    pq.push(State {
        cost: 0,
        position: start,
        direction: 0,
    });

    while let Some(State {
        cost,
        position,
        direction,
    }) = pq.pop()
    {
        let old_cost = dist[&(position, direction)];
        if cost > old_cost {
            continue;
        }

        for (nd, &(dx, dy)) in directions.iter().enumerate() {
            let nr = position.0 as isize + dx;
            let nc = position.1 as isize + dy;
            if nr < 0 || nc < 0 {
                continue;
            }
            let new_pos = (nr as usize, nc as usize);
            if new_pos.0 >= map.len() || new_pos.1 >= map[0].len() {
                continue;
            }
            if map[new_pos.0][new_pos.1] == Elem::Wall {
                continue;
            }

            let mut new_cost = cost + 1;
            if direction != nd {
                new_cost += 1000;
            }

            let old_dist = dist.get(&(new_pos, nd)).cloned().unwrap_or(u32::MAX);
            if new_cost < old_dist {
                dist.insert((new_pos, nd), new_cost);
                pq.push(State {
                    cost: new_cost,
                    position: new_pos,
                    direction: nd,
                });
            }
        }
    }

    // Determine best_cost
    let mut best_cost = u32::MAX;
    for d in 0..4 {
        if let Some(&c) = dist.get(&(goal, d)) {
            if c < best_cost {
                best_cost = c;
            }
        }
    }
    if best_cost == u32::MAX {
        return None;
    }

    let start_dist = dist.get(&(start, 0)).cloned().unwrap_or(u32::MAX);
    if start_dist > best_cost {
        return Some(0);
    }

    let mut stack = vec![];
    let mut best_tiles = HashSet::new();
    // Initial state
    let mut initial_visited = HashSet::new();
    initial_visited.insert(start);
    stack.push(P2State {
        cost: 0,
        position: start,
        direction: 0,
        visited: initial_visited,
    });

    while let Some(P2State {
        cost,
        position,
        direction,
        visited,
    }) = stack.pop()
    {
        if position == goal {
            let goal_dist = dist[&(position, direction)];
            if goal_dist == best_cost {
                for &pos in &visited {
                    best_tiles.insert(pos);
                }
            }
            continue;
        }

        let base_dist = dist
            .get(&(position, direction))
            .cloned()
            .unwrap_or(u32::MAX);
        if base_dist > best_cost {
            continue;
        }

        for (nd, &(dx, dy)) in directions.iter().enumerate() {
            let nr = position.0 as isize + dx;
            let nc = position.1 as isize + dy;
            if nr < 0 || nc < 0 {
                continue;
            }
            let new_pos = (nr as usize, nc as usize);
            if new_pos.0 >= map.len() || new_pos.1 >= map[0].len() {
                continue;
            }
            if map[new_pos.0][new_pos.1] == Elem::Wall {
                continue;
            }

            let mut step_cost = 1;
            if direction != nd {
                step_cost += 1000;
            }

            if let Some(&next_dist) = dist.get(&(new_pos, nd)) {
                if base_dist + step_cost == next_dist && next_dist <= best_cost {
                    let mut new_visited = visited.clone();
                    new_visited.insert(new_pos);
                    stack.push(P2State {
                        cost: cost + step_cost,
                        position: new_pos,
                        direction: nd,
                        visited: new_visited,
                    });
                }
            }
        }
    }

    Some(best_tiles.len() as u32)
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
