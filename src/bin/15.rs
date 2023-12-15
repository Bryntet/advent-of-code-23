advent_of_code::solution!(15);
use itertools::Itertools;
use std::collections::HashMap;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Operation {
    Remove {
        label: String,
        box_id: u32,
    },
    Replace {
        label: String,
        box_id: u32,
        focal_length: u8,
    },
}

impl Operation {
    fn parse_operation(input: &mut Chars, label: Vec<char>) -> Option<Self> {
        let label = label.iter().join("");
        let box_id = calculate_box_id(&label);
        match input.next() {
            Some('-') => Some(Operation::Remove {
                label: label.to_string(),
                box_id,
            }),
            Some('=') => Some(Operation::Replace {
                focal_length: input.join("").parse::<u8>().ok()?,
                label: label.to_string(),
                box_id,
            }),
            _ => None,
        }
    }

    fn new(input: &mut Chars) -> Option<Self> {
        let mut label: Vec<char> = Vec::new();
        let mut c_list = input.clone();
        let mut current = c_list.next()?;
        loop {
            label.push(current);
            if c_list.clone().next()? == '=' || c_list.clone().next()? == '-' {
                break;
            }
            current = c_list.next().unwrap();
        }
        let op = Operation::parse_operation(&mut c_list, label);
        op
    }
}

fn calculate_box_id(input: &str) -> u32 {
    let mut res = 0;
    input.chars().map(|c| (c as u8) as u32).for_each(|v| {
        res += v;
        res *= 17;
        res %= 256;
    });
    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let x: u32 = input
        .lines()
        .join("")
        .split(',')
        .map(calculate_box_id)
        .sum();
    Some(x)
}

#[derive(Debug)]
struct Lens {
    focal_length: u8,
    slot: usize,
}

impl Lens {
    fn new(focal_length: u8, slot: usize) -> Self {
        Self { focal_length, slot }
    }
}

fn max_slot(set: &HashMap<String, Lens>) -> Option<usize> {
    set.iter().map(|(_, lens)| lens.slot).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let labels = input
        .lines()
        .join("")
        .split(',')
        .filter_map(|s| Operation::new(&mut s.chars()))
        .collect_vec();

    let mut map: HashMap<u32, HashMap<String, Lens>> = HashMap::new();
    for label in labels {
        match label {
            Operation::Remove { box_id, label } => {
                if let Some(m) = map.get_mut(&box_id) {
                    m.remove(&label);
                }
            }
            Operation::Replace {
                box_id,
                focal_length,
                label,
            } => {
                map.entry(box_id)
                    .and_modify(|set| {
                        let maybe_set = set.get_mut(&label);
                        if let Some(val) = maybe_set {
                            val.focal_length = focal_length;
                        } else {
                            set.insert(
                                label.clone(),
                                Lens::new(focal_length, max_slot(set).unwrap_or(0) + 1),
                            );
                        }
                    })
                    .or_insert({
                        let mut set = HashMap::new();
                        set.insert(label, Lens::new(focal_length, 1));
                        set
                    });
            }
        }
        fix_slots(&mut map);
    }

    //dbg!(&map);
    Some(
        map.iter()
            .map(|(box_id, lens)| {
                lens.iter()
                    .map(|(_, lens)| {
                        (box_id + 1) * (lens.slot as u32 + 1) * lens.focal_length as u32
                    })
                    .sum::<u32>()
            })
            .sum(),
    )
}

fn fix_slots(m: &mut HashMap<u32, HashMap<String, Lens>>) {
    m.iter_mut().for_each(|(_, map)| {
        let mut slots: Vec<usize> = map.values().map(|lens| lens.slot).collect();
        slots.sort_unstable();
        for (i, slot) in slots.iter().enumerate() {
            map.values_mut()
                .find(|lens| lens.slot == *slot)
                .unwrap()
                .slot = i;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
