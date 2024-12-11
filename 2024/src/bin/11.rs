use cached::proc_macro::cached;

advent_of_code::solution!(11);

#[cached]
fn blink(n: u64, b: u64, target: u64) -> u64 {
    if b == target {
        return 1;
    }
    if n == 0 {
        return blink(1, b + 1, target);
    }
    let mut left = n.to_string();
    if left.len() % 2 == 0 {
        let right = left.split_off(left.len() / 2);
        return blink(left.parse().unwrap(), b + 1, target)
            + blink(right.parse().unwrap(), b + 1, target);
    }
    blink(n * 2024, b + 1, target)
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = input.split(" ").map(|m| m.parse::<u64>().unwrap());
    stones
        .map(|stone| blink(stone, 0, 25))
        .reduce(|acc, v| acc + v)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = input.split(" ").map(|m| m.parse::<u64>().unwrap());
    stones
        .map(|stone| blink(stone, 0, 75))
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
