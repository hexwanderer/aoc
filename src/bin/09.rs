use std::collections::HashMap;

advent_of_code::solution!(9);

#[derive(Debug, Clone)]
enum Component {
    File(u64),
    Free,
}

#[derive(Debug, Clone)]
struct FileMap {
    raw: Vec<Component>,
    id_locations: HashMap<u64, u64>,
    id_sizes: HashMap<u64, u64>,
}

impl FileMap {
    fn new(input: &str) -> Self {
        let (mut is_file, mut last_id, mut last_pos, mut loc_map, mut size_map) = (
            true,
            0,
            0,
            HashMap::<u64, u64>::new(),
            HashMap::<u64, u64>::new(),
        );
        let raw = input
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
            .collect::<Vec<_>>();
        Self {
            raw,
            id_locations: loc_map,
            id_sizes: size_map,
        }
    }

    fn leftmost_free_space(&self) -> Option<usize> {
        let mut space_on_left = None;
        for (i, component) in self.raw.iter().enumerate() {
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

    fn defragment_full(&mut self) {
        while let Some(loc) = self.leftmost_free_space() {
            for i in (0..self.raw.len()).rev() {
                if let Component::File(_) = self.raw[i] {
                    self.raw.swap(loc, i);
                    break;
                }
            }
        }
    }

    fn leftmost_contiguous_free_space(&self, target_size: u64) -> Option<usize> {
        let (mut space_pos, mut size) = (None, 0);
        for (i, component) in self.raw.iter().enumerate() {
            match component {
                Component::File(_) => {
                    if space_pos.is_some() {
                        if size >= target_size {
                            return space_pos;
                        }
                        space_pos = None;
                        size = 0;
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

    fn defragment_files(&mut self) {
        for id in (0..=*self.id_locations.keys().max().unwrap()).rev() {
            let file_size = self.id_sizes[&id];
            let file_loc = self.id_locations[&id] as usize;
            if let Some(space_pos) = self.leftmost_contiguous_free_space(file_size) {
                if space_pos < file_loc {
                    for empty_index in 0..file_size as usize {
                        self.raw
                            .swap(space_pos + empty_index, file_loc + empty_index as usize);
                    }
                }
            }
        }
    }

    fn checksum(&self) -> Option<u64> {
        self.raw
            .iter()
            .enumerate()
            .filter_map(|(pos, file)| match file {
                Component::File(id) => Some(pos as u64 * id),
                Component::Free => None,
            })
            .reduce(|acc, v| acc + v)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut filemap = FileMap::new(input);
    filemap.defragment_full();
    filemap.checksum()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut filemap = FileMap::new(input);
    filemap.defragment_files();
    filemap.checksum()
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
