advent_of_code::solution!(7);

fn generate_sequences(
    numbers: &[u64],
    index: usize,
    running: u64,
    target: u64,
    concat_mode: bool,
) -> Vec<u64> {
    if index == numbers.len() {
        return vec![running];
    }
    if running > target {
        return vec![u64::MAX]; // Exceeding target
    }

    let next_number = numbers[index];
    let mut results = Vec::new();

    // Proceed with addition and multiplication, and concatenation if in concat_mode
    results.extend(generate_sequences(
        numbers,
        index + 1,
        running + next_number,
        target,
        concat_mode,
    ));

    results.extend(generate_sequences(
        numbers,
        index + 1,
        running * next_number,
        target,
        concat_mode,
    ));

    if concat_mode {
        let num_digits = (next_number as f64).log(10.0).floor() as u64 + 1;
        let concat_number = running * 10u64.pow(num_digits as u32) + next_number;
        results.extend(generate_sequences(
            numbers,
            index + 1,
            concat_number,
            target,
            concat_mode,
        ));
    }

    results
}

fn process_line(line: &str, concat_mode: bool) -> Option<u64> {
    let (left, right) = line.split_once(":").unwrap();
    let pieces = right
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let target = left.parse::<u64>().unwrap();

    let targets = generate_sequences(&pieces, 0, 0, target, concat_mode);
    if targets.iter().any(|&val| val == target) {
        Some(target)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .filter_map(|line| process_line(line, false))
        .reduce(|acc, v| acc + v)
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .lines()
        .filter_map(|line| process_line(line, true))
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
