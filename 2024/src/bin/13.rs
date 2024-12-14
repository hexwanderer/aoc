advent_of_code::solution!(13);

fn solve(x: (i64, i64), y: (i64, i64), z: (i64, i64)) -> Option<i64> {
    let b = (z.1 * x.0 - z.0 * x.1) / (y.1 * x.0 - y.0 * x.1);
    let a = (z.0 - b * y.0) / x.0;
    if (x.0 * a + y.0 * b, x.1 * a + y.1 * b) != (z.0, z.1) {
        None
    } else {
        Some(3 * a + b)
    }
}

fn parse_input(input: &str) -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    input
        .split("\n\n") // Separate blocks for each machine
        .filter(|block| !block.trim().is_empty())
        .map(|block| {
            let mut lines = block.lines();

            // Parse Button A
            let button_a = lines
                .next()
                .expect("Missing Button A line")
                .split(&[',', ' ', 'X', 'Y', '+'][..])
                .filter_map(|v| v.parse::<i64>().ok())
                .collect::<Vec<i64>>();

            // Parse Button B
            let button_b = lines
                .next()
                .expect("Missing Button B line")
                .split(&[',', ' ', 'X', 'Y', '+'][..])
                .filter_map(|v| v.parse::<i64>().ok())
                .collect::<Vec<i64>>();

            // Parse Prize
            let prize = lines
                .next()
                .expect("Missing Prize line")
                .split(&[',', ' ', 'X', 'Y', '='][..])
                .filter_map(|v| v.parse::<i64>().ok())
                .collect::<Vec<i64>>();

            // Collect results into tuples
            (
                (button_a[0], button_a[1]),
                (button_b[0], button_b[1]),
                (prize[0], prize[1]),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let tuples = parse_input(input);
    tuples
        .iter()
        .filter_map(|row| solve(row.0, row.1, row.2))
        .reduce(|acc, v| acc + v)
}

pub fn part_two(input: &str) -> Option<i64> {
    let tuples = parse_input(input);
    tuples
        .iter()
        .filter_map(|row| {
            solve(
                row.0,
                row.1,
                (row.2 .0 + 10000000000000, row.2 .1 + 10000000000000),
            )
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
