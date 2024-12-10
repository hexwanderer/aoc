use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

struct PageRules {
    before: Vec<u32>,
    after: Vec<u32>,
}

impl Default for PageRules {
    fn default() -> Self {
        PageRules {
            before: vec![],
            after: vec![],
        }
    }
}

fn define_rules(rules: &str) -> HashMap<u32, PageRules> {
    let mut page_rules = HashMap::<u32, PageRules>::new();
    rules.lines().into_iter().for_each(|rule| {
        let (left, right) = rule.split_once("|").unwrap();
        let (left_page, right_page): (u32, u32) = (left.parse().unwrap(), right.parse().unwrap());

        page_rules
            .entry(left_page)
            .or_insert(PageRules::default())
            .after
            .push(right_page);
        page_rules
            .entry(right_page)
            .or_insert(PageRules::default())
            .before
            .push(left_page);
    });

    page_rules
}

fn is_valid_update(update: &Vec<u32>, page_rules: &HashMap<u32, PageRules>) -> bool {
    for i in 0..update.len() {
        let rules = page_rules.get(&update[i]).unwrap();

        let must_before = &update[0..i];
        let must_after = &update[i + 1..update.len()];

        if rules
            .before
            .iter()
            .any(|comes_before| must_after.contains(comes_before))
        {
            return false;
        }
        if rules
            .after
            .iter()
            .any(|comes_after| must_before.contains(comes_after))
        {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let updates_vec = updates.lines().map(|l| {
        l.split(",")
            .map(|c| c.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    });

    let page_rules = define_rules(rules);

    updates_vec
        .filter_map(|update| {
            if !is_valid_update(&update, &page_rules) {
                None
            } else {
                Some(update[update.len() / 2])
            }
        })
        .reduce(|acc, v| acc + v)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let updates_vec = updates.lines().map(|l| {
        l.split(",")
            .map(|c| c.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    });

    let page_rules = define_rules(rules);

    updates_vec
        .filter_map(|update| {
            if is_valid_update(&update, &page_rules) {
                None
            } else {
                let mut update = update;
                update.sort_by(|a, b| {
                    if page_rules.get(a).unwrap().before.contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                Some(update[update.len() / 2])
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
