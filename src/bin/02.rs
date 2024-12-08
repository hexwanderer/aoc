advent_of_code::solution!(2);

fn solve_line(line: &Vec<isize>, indices: &Vec<usize>, is_asc: bool) -> bool {
    indices
        .windows(2)
        .map(|pair| {
            let (l, r) = (pair[0], pair[1]);
            let (ll, rr) = (line[l], line[r]);
            if is_asc {
                ll < rr && (1..=3).contains(&(rr - ll))
            } else {
                rr < ll && (1..=3).contains(&(ll - rr))
            }
        })
        .all(|pred| pred)
}

pub fn part_one(input: &str) -> Option<u32> {
    // Split the input into lines
    let lines: Vec<&str> = input.lines().collect();

    // Process each line into two vectors of integers
    let puzzle_lines = lines.iter().map(|line| {
        line.split(" ")
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>()
    });

    // Solve the problem
    let result = puzzle_lines
        .map(|line| {
            (solve_line(&line, &(0..line.len()).collect(), true)
                || solve_line(&line, &(0..line.len()).collect(), false)) as u32
        })
        .reduce(|acc, v| acc + v);

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    // Split the input into lines
    let lines: Vec<&str> = input.lines().collect();

    // Process each line into two vectors of integers
    let puzzle_lines = lines.iter().map(|line| {
        line.split(" ")
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>()
    });

    // Solve the problem
    let result = puzzle_lines
        .map(|line| {
            let indices = 0..line.len();
            let skips: Vec<Vec<usize>> = indices
                .clone()
                .enumerate()
                .map(|(i, _)| {
                    indices
                        .clone()
                        .enumerate()
                        .filter(|(j, _)| j != &i)
                        .map(|(_, v)| v.clone())
                        .collect()
                })
                .collect();
            for skip in skips {
                if solve_line(&line, &skip, true) || solve_line(&line, &skip, false) {
                    return 1;
                }
            }
            0
        })
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
