use std::collections::HashSet;
use itertools::Itertools;
advent_of_code::solution!(4);



struct Card {
    winners: HashSet<u32>,
    nums: Vec<u32>
}

impl Card {
    fn new(winners: Vec<u32>, nums: Vec<u32>) -> Self {
        let mut new_winners: HashSet<u32> = HashSet::new();
        winners.iter().for_each(|w| { new_winners.insert(w.to_owned()); });
        Card {
            winners:new_winners,
            nums
        }
    }

    fn amount_of_winners(&self) -> usize {
        self.nums.iter().filter(|n|self.winners.contains(n)).collect_vec().len()
    }

    fn get_points(&self) -> usize {
        dbg!(self.amount_of_winners());
        let w = self.amount_of_winners();
        if w == 1 {
            1
        } else if w > 1 {
            (w-1).pow(2)
        } else {
            0
        }
    }
}


pub fn part_one(input: &str) -> Option<usize> {
    let mut cards: Vec<Card> = input
        .lines()
        .filter_map(|l| {
            let a: Option<(Vec<u32>, Vec<u32>)> = l
                .split(": ")
                .last()
                .unwrap()
                .split(" | ")
                .map(|s| {
                    s.split(' ')
                        .filter_map(|num| num.parse::<u32>().ok())
                        .collect_vec()
                })
                .collect_tuple();
            a
        })
        .map(|c|{
            Card::new(c.0,c.1)
        })
        .collect_vec();

    Some(cards.iter().map(|c|c.get_points()).sum::<usize>())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
