advent_of_code::solution!(7);

fn generate_sequences(
    numbers: &Vec<u64>,
    index: usize,
    running: u64,
    target: &u64,
    concat_mode: bool,
) -> Vec<u64> {
    if index == numbers.len() {
        return vec![running];
    }
    if running > *target {
        return vec![u64::MAX];
    }

    let mut results = vec![];
    if index == 0 {
        results.extend(generate_sequences(
            numbers,
            index + 1,
            numbers[index],
            target,
            concat_mode,
        ));
    } else {
        results.extend(generate_sequences(
            numbers,
            index + 1,
            running + numbers[index],
            target,
            concat_mode,
        ));
        results.extend(generate_sequences(
            numbers,
            index + 1,
            running * numbers[index],
            target,
            concat_mode,
        ));
        if concat_mode {
            let concat_number = format!("{}{}", running, numbers[index])
                .parse::<u64>()
                .unwrap();
            results.extend(generate_sequences(
                numbers,
                index + 1,
                concat_number,
                target,
                concat_mode,
            ));
        }
    }
    results
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let (left, right) = line.split_once(":").unwrap();
            let pieces = right
                .trim()
                .split(" ")
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let target = left.parse::<u64>().unwrap();
            let targets = generate_sequences(&pieces, 0, 0, &target, false);
            if targets.iter().any(|val| val == &target) {
                target
            } else {
                0
            }
        })
        .reduce(|acc, v| acc + v)
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let (left, right) = line.split_once(":").unwrap();
            let pieces = right
                .trim()
                .split(" ")
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let target = left.parse::<u64>().unwrap();
            let targets = generate_sequences(&pieces, 0, 0, &target, true);
            if targets.iter().any(|val| val == &target) {
                target
            } else {
                0
            }
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
