use std::collections::HashMap;

advent_of_code::solution!(4);

fn get_neighbors(x: usize, y: usize, bx: usize, by: usize) -> Vec<Vec<(usize, usize)>> {
    let mut results = vec![];
    if y + 3 < by {
        let mut fwd_horizontal = vec![];
        for yy in y..=y + 3 {
            fwd_horizontal.push((x, yy));
        }
        results.push(fwd_horizontal);
    }

    if y >= 3 {
        let mut bwd_horizontal = vec![];
        for yy in (y - 3..=y).rev() {
            bwd_horizontal.push((x, yy));
        }
        results.push(bwd_horizontal);
    }

    if x + 3 < bx {
        let mut dwd_vertical = vec![];
        for xx in x..=x + 3 {
            dwd_vertical.push((xx, y));
        }
        results.push(dwd_vertical);
    }

    if x >= 3 {
        let mut uwd_vertical = vec![];
        for xx in (x - 3..=x).rev() {
            uwd_vertical.push((xx, y));
        }
        results.push(uwd_vertical);
    }

    if x >= 3 && y >= 3 {
        let mut tl_diagonal = vec![];
        for (xx, yy) in ((x - 3..=x).rev()).zip((y - 3..=y).rev()) {
            tl_diagonal.push((xx, yy));
        }
        results.push(tl_diagonal);
    }

    if x + 3 < bx && y >= 3 {
        let mut tr_diagonal = vec![];
        for (xx, yy) in (x..=x + 3).zip((y - 3..=y).rev()) {
            tr_diagonal.push((xx, yy));
        }
        results.push(tr_diagonal);
    }

    if x >= 3 && y + 3 < by {
        let mut bl_diagonal = vec![];
        for (xx, yy) in ((x - 3..=x).rev()).zip(y..=y + 3) {
            bl_diagonal.push((xx, yy));
        }
        results.push(bl_diagonal);
    }

    if x + 3 < bx && y + 3 < by {
        let mut br_diagonal = vec![];
        for (xx, yy) in (x..=x + 3).zip(y..=y + 3) {
            br_diagonal.push((xx, yy));
        }
        results.push(br_diagonal);
    }

    return results;
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let xmas_index: HashMap<usize, char> = [(0, 'X'), (1, 'M'), (2, 'A'), (3, 'S')]
        .into_iter()
        .collect();

    let mut result = 0;
    for (x, row) in map.iter().enumerate() {
        for (y, val) in row.iter().enumerate() {
            if val == &'X' {
                let neighbors = get_neighbors(x, y, map.len(), row.len());
                for neighbor in neighbors {
                    if neighbor
                        .iter()
                        .enumerate()
                        .map(|(i, (r, c))| {
                            let character = map[*r][*c];
                            return &character == xmas_index.get(&i).unwrap();
                        })
                        .all(|pred| pred)
                    {
                        result += 1
                    }
                }
            }
        }
    }

    Some(result)
}

fn get_neighbors_two(
    x: usize,
    y: usize,
    bx: usize,
    by: usize,
) -> Option<Vec<(Vec<(usize, usize)>, Vec<(usize, usize)>)>> {
    if x > 0 && x + 1 < bx && y > 0 && y + 1 < by {
        Some(vec![
            (
                vec![(x - 1, y - 1), (x, y), (x + 1, y + 1)],
                vec![(x - 1, y + 1), (x, y), (x + 1, y - 1)],
            ),
            (
                vec![(x + 1, y + 1), (x, y), (x - 1, y - 1)],
                vec![(x - 1, y + 1), (x, y), (x + 1, y - 1)],
            ),
            (
                vec![(x - 1, y - 1), (x, y), (x + 1, y + 1)],
                vec![(x + 1, y - 1), (x, y), (x - 1, y + 1)],
            ),
            (
                vec![(x + 1, y + 1), (x, y), (x - 1, y - 1)],
                vec![(x + 1, y - 1), (x, y), (x - 1, y + 1)],
            ),
        ])
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let xmas_index: HashMap<usize, char> = [(0, 'M'), (1, 'A'), (2, 'S')].into_iter().collect();

    let mut result = 0;
    for (x, row) in map.iter().enumerate() {
        for (y, val) in row.iter().enumerate() {
            if val == &'A' {
                if let Some(diagonals) = get_neighbors_two(x, y, map.len(), row.len()) {
                    for (one_diagonal, two_diagonal) in diagonals {
                        if one_diagonal
                            .iter()
                            .enumerate()
                            .map(|(i, (r, c))| {
                                let character = map[*r][*c];
                                return &character == xmas_index.get(&i).unwrap();
                            })
                            .all(|pred| pred)
                            && two_diagonal
                                .iter()
                                .enumerate()
                                .map(|(i, (r, c))| {
                                    let character = map[*r][*c];
                                    return &character == xmas_index.get(&i).unwrap();
                                })
                                .all(|pred| pred)
                        {
                            result += 1
                        }
                    }
                }
            }
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
