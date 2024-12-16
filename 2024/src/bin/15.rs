advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Elem {
    Empty,
    Box,
    Wall,
    Robot,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum WideElem {
    Empty,
    LeftBox,
    RightBox,
    Wall,
    Robot,
}

impl From<char> for Elem {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '@' => Self::Robot,
            '.' => Self::Empty,
            'O' => Self::Box,
            _ => panic!("unrecognized element"),
        }
    }
}

impl From<char> for WideElem {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '@' => Self::Robot,
            '.' => Self::Empty,
            '[' => Self::LeftBox,
            ']' => Self::RightBox,
            _ => panic!("unrecognized element"),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Up,
    Left,
    Right,
    Down,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            '<' => Self::Left,
            '>' => Self::Right,
            'v' => Self::Down,
            '^' => Self::Up,
            _ => panic!("unrecognized instruction"),
        }
    }
}

impl Instruction {
    fn make_move(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

type Warehouse = Vec<Vec<Elem>>;

type WideWarehouse = Vec<Vec<WideElem>>;

fn process(input: &str) -> (Warehouse, (isize, isize), Vec<Instruction>) {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let warehouse: Warehouse = map
        .lines()
        .map(|row| row.chars().map(|ch| ch.into()).collect::<Vec<Elem>>())
        .collect();
    let processed_instructions: Vec<Instruction> = instructions
        .trim()
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| ch.into())
        .collect();

    let mut robot_pos = None;
    for (r, row) in warehouse.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == Elem::Robot {
                robot_pos = Some((r as isize, c as isize));
            }
        }
    }

    (warehouse, robot_pos.unwrap(), processed_instructions)
}

fn process_wide(
    map: &str,
    instructions: &str,
) -> (WideWarehouse, (isize, isize), Vec<Instruction>) {
    let warehouse: WideWarehouse = map
        .lines()
        .map(|row| row.chars().map(|ch| ch.into()).collect::<Vec<WideElem>>())
        .collect();
    let processed_instructions: Vec<Instruction> = instructions
        .trim()
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| ch.into())
        .collect();

    let mut robot_pos = None;
    for (r, row) in warehouse.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == WideElem::Robot {
                robot_pos = Some((r as isize, c as isize));
            }
        }
    }

    (warehouse, robot_pos.unwrap(), processed_instructions)
}

fn move_robot(from: &mut (isize, isize), direction: &Instruction, on: &mut Warehouse) {
    let step = direction.make_move();
    let mut next_move = (from.0 + step.0, from.1 + step.1);

    let mut boxes_to_move = Vec::new();

    // Track all boxes to be moved
    while on[next_move.0 as usize][next_move.1 as usize] == Elem::Box {
        let after_box = (next_move.0 + step.0, next_move.1 + step.1);

        // Stop if the destination for the next box is not empty
        if on[after_box.0 as usize][after_box.1 as usize] == Elem::Wall {
            return; // Abort movement
        }

        boxes_to_move.push(next_move);
        next_move = after_box;
    }

    // Move boxes in reverse order to avoid overwriting
    for &box_pos in boxes_to_move.iter().rev() {
        let new_box_pos = (box_pos.0 + step.0, box_pos.1 + step.1);
        on[new_box_pos.0 as usize][new_box_pos.1 as usize] = Elem::Box;
        on[box_pos.0 as usize][box_pos.1 as usize] = Elem::Empty;
    }

    // Stop if the robot's next position is not empty
    if on[next_move.0 as usize][next_move.1 as usize] == Elem::Wall {
        return; // Abort movement
    }

    // Move the robot
    on[from.0 as usize][from.1 as usize] = Elem::Empty;
    *from = (from.0 + step.0, from.1 + step.1);
    on[from.0 as usize][from.1 as usize] = Elem::Robot;
}

fn move_wide_robot(from: &mut (isize, isize), direction: &Instruction, on: &mut WideWarehouse) {
    let step = direction.make_move();
    let next_robot_pos = (from.0 + step.0, from.1 + step.1);

    // If horizontal, similar to previous
    if step.0 == 0 {
        let mut pos = next_robot_pos;
        let mut size = 1;
        let w = on[0].len();
        let h = on.len();

        // Find next wall or empty space
        while pos.0 >= 0
            && pos.0 < h as isize
            && pos.1 >= 0
            && pos.1 < w as isize
            && on[pos.0 as usize][pos.1 as usize] != WideElem::Empty
            && on[pos.0 as usize][pos.1 as usize] != WideElem::Wall
        {
            pos = (pos.0 + step.0, pos.1 + step.1);
            size += 1;
        }

        if pos.0 < 0 || pos.0 >= h as isize || pos.1 < 0 || pos.1 >= w as isize {
            return; // out of bounds
        }

        // If empty space found, push boxes one step
        if on[pos.0 as usize][pos.1 as usize] == WideElem::Empty {
            let mut prev = WideElem::Empty;
            let mut p = (from.0 + step.0, from.1 + step.1);
            for _ in 0..size {
                let curr = on[p.0 as usize][p.1 as usize];
                on[p.0 as usize][p.1 as usize] = prev;
                prev = curr;
                p = (p.0 + step.0, p.1 + step.1);
            }
            // Move robot
            on[from.0 as usize][from.1 as usize] = WideElem::Empty;
            *from = next_robot_pos;
            on[from.0 as usize][from.1 as usize] = WideElem::Robot;
        }
    } else {
        // If vertical, we need to find boxes touched
        let w = on[0].len();
        let h = on.len();

        // If front is empty, just move robot
        if next_robot_pos.0 >= 0
            && next_robot_pos.0 < h as isize
            && next_robot_pos.1 >= 0
            && next_robot_pos.1 < w as isize
            && on[next_robot_pos.0 as usize][next_robot_pos.1 as usize] == WideElem::Empty
        {
            on[from.0 as usize][from.1 as usize] = WideElem::Empty;
            *from = next_robot_pos;
            on[from.0 as usize][from.1 as usize] = WideElem::Robot;
            return;
        }

        let mut todo = Vec::with_capacity(50);
        // Dummy item
        todo.push((!0, !0));
        todo.push(*from);

        let mut index = 1;

        while index < todo.len() {
            let current = todo[index];
            index += 1;

            let next = (current.0 + step.0, current.1 + step.1);
            if next.0 < 0 || next.0 >= h as isize || next.1 < 0 || next.1 >= w as isize {
                return;
            }

            match on[next.0 as usize][next.1 as usize] {
                WideElem::LeftBox => {
                    let right = (next.0, next.1 + 1);
                    if right.1 as usize >= w {
                        return;
                    }
                    if on[right.0 as usize][right.1 as usize] != WideElem::RightBox {
                        return; // malformed box
                    }
                    let after_left = (next.0 + step.0, next.1 + step.1);
                    let after_right = (right.0 + step.0, right.1 + step.1);
                    if on[after_left.0 as usize][after_left.1 as usize] == WideElem::Wall
                        || on[after_right.0 as usize][after_right.1 as usize] == WideElem::Wall
                    {
                        return;
                    }
                    if (todo.len() < 4) || (todo[todo.len() - 2] != next) {
                        todo.push(next);
                        todo.push(right);
                    }
                }
                WideElem::RightBox => {
                    let left = (next.0, next.1 - 1);
                    if left.1 < 0 {
                        return;
                    }
                    if on[left.0 as usize][left.1 as usize] != WideElem::LeftBox {
                        return; // malformed box
                    }
                    let after_left = (left.0 + step.0, left.1 + step.1);
                    let after_right = (next.0 + step.0, next.1 + step.1);
                    if on[after_left.0 as usize][after_left.1 as usize] == WideElem::Wall
                        || on[after_right.0 as usize][after_right.1 as usize] == WideElem::Wall
                    {
                        return;
                    }
                    if (todo.len() < 4) || (todo[todo.len() - 2] != left) {
                        todo.push(left);
                        todo.push(next);
                    }
                }
                WideElem::Wall => return,
                WideElem::Empty | WideElem::Robot => {}
            }
        }

        // Move boxes in reverse order
        for &p in todo[2..].iter().rev() {
            let new_p = (p.0 + step.0, p.1 + step.1);
            on[new_p.0 as usize][new_p.1 as usize] = on[p.0 as usize][p.1 as usize];
            on[p.0 as usize][p.1 as usize] = WideElem::Empty;
        }

        // Move robot
        on[from.0 as usize][from.1 as usize] = WideElem::Empty;
        *from = next_robot_pos;
        on[from.0 as usize][from.1 as usize] = WideElem::Robot;
    }
}

fn gps_coordinate(point: (usize, usize)) -> u32 {
    (point.0 as u32) * 100 + point.1 as u32
}

fn widen_map(input: &str) -> String {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|ch| match ch {
                    '#' => "##".to_owned(),
                    'O' => "[]".to_owned(),
                    '.' => "..".to_owned(),
                    '@' => "@.".to_owned(),
                    _ => ch.to_string(),
                })
                .collect::<Vec<_>>()
                .concat()
                + "\n"
        })
        .collect::<Vec<_>>()
        .concat()
        .to_string()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut warehouse, mut robot_pos, instructions) = process(input);
    for instruction in instructions.iter() {
        move_robot(&mut robot_pos, instruction, &mut warehouse);
    }
    warehouse
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(|(c, val)| {
                    if *val == Elem::Box {
                        gps_coordinate((r, c))
                    } else {
                        0
                    }
                })
                .reduce(|acc, v| acc + v)
                .unwrap()
        })
        .reduce(|acc, v| acc + v)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let wide_map = widen_map(map);
    let (mut warehouse, mut robot_pos, instructions) = process_wide(&wide_map, instructions);
    for instruction in instructions.iter() {
        move_wide_robot(&mut robot_pos, instruction, &mut warehouse);
    }
    warehouse
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(|(c, val)| {
                    if *val == WideElem::LeftBox {
                        gps_coordinate((r, c))
                    } else {
                        0
                    }
                })
                .reduce(|acc, v| acc + v)
                .unwrap()
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
    fn test_15_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
