advent_of_code::solution!(1);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let nums = l
                    .chars()
                    .filter_map(|c| {
                        if c.is_ascii_digit() {
                            Some(c.to_digit(10).unwrap())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<u32>>();
                Some(nums.first()? * 10 + nums.last()?)
            })
            .sum(),
    )
}

enum NumberResult {
    Done(u32),
    Partial,
    NoMatch,
}

fn can_become_number(s: &str) -> NumberResult {
    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    if numbers.iter().any(|&num| num == s) {
        NumberResult::Done(match s {
            "zero" => 0,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => panic!("Not a number"),
        })
    } else if numbers.iter().any(|&num| num.starts_with(s)) {
        NumberResult::Partial
    } else {
        NumberResult::NoMatch
    }
}

fn iterate_until_checked(potential_num: &mut String) {
    while !potential_num.is_empty()
        && matches!(can_become_number(potential_num), NumberResult::NoMatch)
    {
        potential_num.remove(0);
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let mut letter_num_check = "".to_string();
                let nums = l
                    .chars()
                    .filter_map(|c| {
                        if c.is_ascii_digit() {
                            letter_num_check = "".to_string();
                            Some(c.to_digit(10).unwrap())
                        } else {
                            letter_num_check.push(c);
                            match can_become_number(&letter_num_check) {
                                NumberResult::Done(n) => {
                                    letter_num_check.remove(0);
                                    iterate_until_checked(&mut letter_num_check);
                                    Some(n)
                                }
                                NumberResult::Partial => None,
                                NumberResult::NoMatch => {
                                    iterate_until_checked(&mut letter_num_check);
                                    None
                                }
                            }
                        }
                    })
                    .collect::<Vec<u32>>();
                Some(nums.first()? * 10 + nums.last()?)
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
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
