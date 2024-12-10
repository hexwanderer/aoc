advent_of_code::solution!(3);

// Special sequences we will consider
const MUL_CHAR: &[u8] = "mul".as_bytes();
const DO_CHAR: &[u8] = "do()".as_bytes();
const DONT_CHAR: &[u8] = "don't()".as_bytes();

pub fn part_one(input: &str) -> Option<u32> {
    // Parse the input bytes
    let input_bytes = input.as_bytes();
    let mut result = 0;

    for index in input_bytes.windows(3).enumerate().filter_map(|(i, set)| {
        // Detect sequences of text with a `mul` at the beginning
        if set == MUL_CHAR {
            Some(i + 3)
        } else {
            None
        }
    }) {
        let (mut left, mut right) = (None, None);
        let (mut paren_left, mut paren_right, mut parsing_right) = (false, false, false);

        for &byte in &input_bytes[index..] {
            match byte {
                b'(' => paren_left = true,
                b')' => paren_right = true,
                b',' => parsing_right = true,
                b'0'..=b'9' => {
                    let digit = (byte - b'0') as u32;
                    if parsing_right {
                        right = Some(right.unwrap_or(0) * 10 + digit);
                    } else {
                        left = Some(left.unwrap_or(0) * 10 + digit);
                    }
                }
                // Any other character in this sequence is invalid...
                _ => break,
            }
        }

        match (paren_left && paren_right && parsing_right, left, right) {
            // Only add if all conditions match.
            (true, Some(left), Some(right)) => result += left * right,
            _ => (),
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Parse the input bytes
    let input_bytes = input.as_bytes();
    let mut result = 0;

    // There are a few special scenarios we need to consider, but
    // ultimately, we want to find ranges between don't()'s and
    // the nearest adjacent do():
    //
    // 1. `encountered_do` is necessary to identify consecutive don't()'s.
    // Without it, given a sequence don't()...don't()...do(), the start
    // of the skip would be updated to the start of the second don't()
    // when we encounter it.
    // 2. `last` is necessary to used to identify consecutive do()'s.
    // Consider a sequence `don't()...do()...do()` -- if we added the
    // range from the last don't() to each do(), we would consider the
    // range between the two do()'s to need to be skipped.
    let mut ranges_to_skip = vec![];
    let mut last = None;
    let mut encountered_do = true;
    let mut start_skip = 0;

    for (index, window) in input_bytes.windows(7).enumerate() {
        if window == DONT_CHAR {
            if encountered_do {
                start_skip = index + DONT_CHAR.len();
            }
            encountered_do = false;
        } else if &window[..DO_CHAR.len()] == DO_CHAR {
            encountered_do = true;
            if let Some(last_start) = last {
                if start_skip == last_start {
                    continue;
                }
            }
            last = Some(start_skip);
            ranges_to_skip.push(start_skip..index + DO_CHAR.len());
        }
    }

    for index in input_bytes.windows(3).enumerate().filter_map(|(i, set)| {
        if set == MUL_CHAR {
            Some(i + 3)
        } else {
            None
        }
    }) {
        // If this index falls in between any of the don't()'s, then skip
        if ranges_to_skip.iter().any(|range| range.contains(&index)) {
            continue;
        }
        let (mut left, mut right) = (None, None);
        let (mut paren_left, mut paren_right, mut parsing_right) = (false, false, false);

        for &byte in &input_bytes[index..] {
            match byte {
                b'(' => paren_left = true,
                b')' => paren_right = true,
                b',' => parsing_right = true,
                b'0'..=b'9' => {
                    let digit = (byte - b'0') as u32;
                    if parsing_right {
                        right = Some(right.unwrap_or(0) * 10 + digit);
                    } else {
                        left = Some(left.unwrap_or(0) * 10 + digit);
                    }
                }
                _ => break,
            }
        }

        match (paren_left && paren_right && parsing_right, left, right) {
            (true, Some(left), Some(right)) => result += left * right,
            _ => (),
        }
    }

    Some(result)
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
