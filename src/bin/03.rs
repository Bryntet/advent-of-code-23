use itertools::Itertools;
use std::collections::HashSet;
use std::ops::RangeInclusive;
advent_of_code::solution!(3);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum PartResult {
    Num(Number),
    Symbol(Symbol),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Number {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
    num: u32,
}

impl Number {
    fn new(stop_x: i32, num_string: &str, y: i32) -> Option<Self> {
        if let Ok(num) = num_string.parse::<u32>() {
            Some(Number {
                x_range: (stop_x - num_string.len() as i32 - 1)..=stop_x,
                y_range: (y - 1..=y + 1),
                num,
            })
        } else {
            None
        }
    }

    fn has_neighbour(&self, other: &Point) -> bool {
        self.x_range.contains(&other.x) && self.y_range.contains(&other.y)
    }
}

fn handle_number(last_num: &mut String, parts: &mut Vec<PartResult>, idx: usize, y: usize) {
    if !last_num.is_empty() {
        let num = Number::new(idx as i32, &(last_num.clone()), y as i32);
        if let Some(num) = num {
            parts.push(PartResult::Num(num));
        }
        last_num.clear()
    }
}
fn make_parts(line: &str, y: usize) -> Vec<PartResult> {
    let mut last_num = "".to_string();
    let mut parts: Vec<PartResult> = vec![];
    (line.to_owned() + ".").char_indices().for_each(|(idx, c)| {
        if c.is_ascii_digit() {
            last_num += &c.to_string();
        } else {
            if c != '.' {
                parts.push(PartResult::Symbol(Symbol {
                    symbol: c.to_string(),
                    point: Point {
                        x: idx as i32,
                        y: y as i32,
                    },
                }));
            }
            handle_number(&mut last_num, &mut parts, idx, y)
        }
    });
    parts
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Symbol {
    symbol: String,
    point: Point,
}

fn get_parts(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let parts = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| make_parts(l, row))
        .collect_vec();
    let mut symbols: Vec<Symbol> = vec![];
    let mut nums: Vec<Number> = vec![];
    parts.iter().for_each(|p| match p {
        PartResult::Num(n) => nums.push(n.clone()),
        PartResult::Symbol(s) => symbols.push(s.clone()),
    });

    (nums, symbols)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (nums, symbols) = get_parts(input);
    let mut set: HashSet<&Number> = HashSet::new();

    Some(
        symbols
            .iter()
            .map(|s| {
                nums.iter()
                    .filter_map(|n| {
                        if n.has_neighbour(&s.point) && !set.contains(n) {
                            set.insert(n);
                            Some(n.num)
                        } else {
                            None
                        }
                    })
                    .sum::<u32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (nums, symbols) = get_parts(input);
    Some(
        symbols
            .iter()
            .filter_map(|s| {
                if s.symbol == "*" {
                    let gears = nums.iter().filter_map(|n| {
                        if n.has_neighbour(&s.point) {
                            Some(n.num)
                        } else {
                            None
                        }
                    });
                    if gears.clone().collect_vec().len() == 2 {
                        Some(gears.product::<u32>())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(925));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6756));
    }
}
