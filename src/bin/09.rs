use std::collections::HashMap;

advent_of_code::solution!(9);

#[derive(Debug, Clone)]
enum Component {
    File(u64),
    Free,
}

fn has_free_space(map: &Vec<Component>) -> Option<usize> {
    let mut space_on_left = None;
    for (i, component) in map.iter().enumerate() {
        match component {
            Component::File(_) => {
                if space_on_left.is_some() {
                    return space_on_left;
                }
            }
            Component::Free => {
                if space_on_left.is_none() {
                    space_on_left = Some(i)
                }
            }
        }
    }
    None
}

fn has_free_space_between(map: &Vec<Component>, target_id: u64, target_size: u64) -> Option<usize> {
    let (mut space_pos, mut size) = (None, 0);
    for (i, component) in map.iter().enumerate() {
        match component {
            Component::File(id) => {
                if space_pos.is_some() {
                    if size >= target_size {
                        return space_pos;
                    }
                    space_pos = None;
                    size = 0;
                }

                if target_id == *id {
                    break;
                }
            }
            Component::Free => {
                if space_pos.is_some() {
                    size += 1;
                } else {
                    (space_pos, size) = (Some(i), 1);
                }
            }
        }
    }
    None
}

fn build_filemap(input: &str) -> (Vec<Component>, u64, HashMap<u64, u64>, HashMap<u64, u64>) {
    let (mut is_file, mut last_id, mut last_pos, mut loc_map, mut size_map) = (
        true,
        0,
        0,
        HashMap::<u64, u64>::new(),
        HashMap::<u64, u64>::new(),
    );
    (
        input
            .chars()
            .into_iter()
            .filter_map(|c| {
                if let Some(digit) = c.to_digit(10) {
                    let component = match is_file {
                        true => {
                            last_id += 1;
                            loc_map.insert(last_id - 1, last_pos);
                            size_map.insert(last_id - 1, digit as u64);
                            Component::File(last_id - 1)
                        }
                        false => Component::Free,
                    };
                    is_file = !is_file;
                    last_pos += digit as u64;
                    Some(vec![component; digit as usize])
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>(),
        last_id,
        loc_map,
        size_map,
    )
}

fn checksum(filemap: &Vec<Component>) -> Option<u64> {
    filemap
        .iter()
        .enumerate()
        .filter_map(|(pos, file)| match file {
            Component::File(id) => Some(pos as u64 * id),
            Component::Free => None,
        })
        .reduce(|acc, v| acc + v)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut filemap, _, _, _) = build_filemap(input);

    while let Some(loc) = has_free_space(&filemap) {
        for i in (0..filemap.len()).rev() {
            if let Component::File(_) = filemap[i] {
                filemap.swap(loc, i);
                break;
            }
        }
    }

    checksum(&filemap)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut filemap, last_id, loc_map, size_map) = build_filemap(input);

    for id in (0..last_id).rev() {
        let file_size = size_map[&id];
        let file_loc = loc_map[&id] as usize;
        if let Some(space_pos) = has_free_space_between(&filemap, id, file_size) {
            for empty_index in 0..file_size as usize {
                filemap.swap(space_pos + empty_index, file_loc + empty_index as usize);
            }
        }
    }

    checksum(&filemap)
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
