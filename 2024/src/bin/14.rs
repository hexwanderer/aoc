advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn parse(input: &str) -> Vec<Self> {
        input
            .lines()
            .map(|line| {
                let (position, velocity) = line.split_once(" ").unwrap();
                let (px, py) = position.trim_start_matches("p=").split_once(",").unwrap();
                let (vx, vy) = velocity.trim_start_matches("v=").split_once(",").unwrap();
                Robot {
                    position: (px.parse().unwrap(), py.parse().unwrap()),
                    velocity: (vx.parse().unwrap(), vy.parse().unwrap()),
                }
            })
            .collect()
    }

    fn make_move(&mut self) {
        self.position.0 = (self.position.0 + self.velocity.0).rem_euclid(101);
        self.position.1 = (self.position.1 + self.velocity.1).rem_euclid(103);
    }

    fn to_map(robots: &Vec<Robot>) -> Vec<Vec<usize>> {
        let mut map: Vec<Vec<usize>> = vec![];
        for _ in 0..103 {
            map.push(vec![0; 103]);
        }
        for robot in robots.iter() {
            map[robot.position.0 as usize][robot.position.1 as usize] = 1;
        }
        map
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut robots = Robot::parse(input);
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.make_move();
        }
    }

    // Grid dimensions
    let width = 101;
    let height = 103;
    let center_x = width / 2;
    let center_y = height / 2;

    // Count robots in each quadrant
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for robot in robots {
        let (x, y) = robot.position;

        if x == center_x as i32 || y == center_y as i32 {
            // Skip robots in the middle row or column
            continue;
        }

        if x < center_x as i32 && y < center_y as i32 {
            top_left += 1;
        } else if x > center_x as i32 && y < center_y as i32 {
            top_right += 1;
        } else if x < center_x as i32 && y > center_y as i32 {
            bottom_left += 1;
        } else if x > center_x as i32 && y > center_y as i32 {
            bottom_right += 1;
        }
    }

    // Calculate safety factor as product of counts in all quadrants
    let safety_factor = top_left * top_right * bottom_left * bottom_right;

    Some(safety_factor)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = Robot::parse(input);
    for num_seconds in 1.. {
        for robot in robots.iter_mut() {
            robot.make_move();
        }
        let map = Robot::to_map(&robots);
        for line in map.iter() {
            for window in line.windows(10) {
                if window == &vec![1; 10] {
                    return Some(num_seconds);
                }
            }
        }
    }
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
