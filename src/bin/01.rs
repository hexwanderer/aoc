use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // Split the input into lines
    let lines: Vec<&str> = input.lines().collect();

    // Process each line into two vectors of integers
    let (mut left, mut right): (Vec<isize>, Vec<isize>) = lines
        .iter()
        .map(|line| {
            let mut numbers_as_string = line.split(" ");
            let first = numbers_as_string.next().unwrap().parse::<isize>().unwrap();
            let second = numbers_as_string.last().unwrap().parse::<isize>().unwrap();
            (first, second)
        })
        .unzip();

    // Sort the files
    left.sort();
    right.sort();

    // Calculate the differences
    let differences = (left.iter().zip(right.iter()))
        .map(|(l, r)| u32::try_from((l - r).abs()).unwrap())
        .reduce(|acc, v| acc + v);

    differences
}

pub fn part_two(input: &str) -> Option<u32> {
    // Split the input into lines
    let lines: Vec<&str> = input.lines().collect();

    // Process each line into two vectors of integers
    let (left, right): (Vec<isize>, Vec<isize>) = lines
        .iter()
        .map(|line| {
            let mut numbers_as_string = line.split(" ");
            let first = numbers_as_string.next().unwrap().parse::<isize>().unwrap();
            let second = numbers_as_string.last().unwrap().parse::<isize>().unwrap();
            (first, second)
        })
        .unzip();

    // Create a count map for how many times each number appears in the right list
    let mut counter = HashMap::new();
    for r in right {
        *counter.entry(r).or_insert(0) += 1;
    }

    let result = left
        .iter()
        .map(|l| u32::try_from(l * *counter.get(l).unwrap_or(&0)).unwrap())
        .reduce(|acc, v| acc + v);

    result
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
