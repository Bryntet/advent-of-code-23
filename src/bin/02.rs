use itertools::MinMaxResult;
advent_of_code::solution!(2);
#[derive(Debug, Clone)]
enum Colour {
    Red(u8),
    Green(u8),
    Blue(u8),
}

impl Colour {
    fn new(colour: &str, number: u8) -> Option<Self> {
        match colour {
            "red" => Some(Self::Red(number)),
            "green" => Some(Self::Green(number)),
            "blue" => Some(Self::Blue(number)),
            _ => None,
        }
    }
}

struct MinimumColour {
    red: u32,
    green: u32,
    blue: u32,
}

fn check_if_colour_below_max(colours: &[Colour], max_red: u8, max_green: u8, max_blue: u8) -> bool {
    colours.iter().all(|colour| match colour {
        Colour::Red(n) => *n <= max_red,
        Colour::Green(n) => *n <= max_green,
        Colour::Blue(n) => *n <= max_blue,
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let mut l = l.split(": ");
                let game: u32 = l.next()?.split(' ').last()?.parse().ok()?;
                let plays = l.next()?;
                plays
                    .split("; ")
                    .map(|set| {
                        set.split(", ")
                            .filter_map(|play| {
                                let mut play = play.split(' ');
                                let number = play.next()?.parse::<u8>().ok()?;
                                let colour = play.next()?;
                                Colour::new(colour, number)
                            })
                            .collect::<Vec<Colour>>()
                    })
                    .all(|set| check_if_colour_below_max(&set, 12, 13, 14))
                    .then_some(game)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let mut l = l.split(": ");
                let game: u32 = l.next()?.split(' ').last()?.parse().ok()?;
                let plays = l
                    .next()?
                    .split("; ")
                    .map(|set| {
                        set.split(", ")
                            .filter_map(|play| {
                                let mut play = play.split(' ');
                                let number = play.next()?.parse::<u8>().ok()?;
                                let colour = play.next()?;
                                Colour::new(colour, number)
                            })
                            .collect::<Vec<Colour>>()
                    })
                    .map(|p| {
                        let mut red = 0;
                        let mut green = 0;
                        let mut blue = 0;
                        for colour in p {
                            match colour {
                                Colour::Red(n) => {
                                    if n > red {
                                        red = n
                                    }
                                }
                                Colour::Green(n) => {
                                    if n > green {
                                        green = n
                                    }
                                }
                                Colour::Blue(n) => {
                                    if n > blue {
                                        blue = n
                                    }
                                }
                            }
                        }
                        MinimumColour {
                            red: red as u32,
                            green: green as u32,
                            blue: blue as u32,
                        }
                    });
                let mut minimum_required = MinimumColour {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                for play in plays {
                    if play.red > minimum_required.red {
                        minimum_required.red = play.red;
                    }
                    if play.green > minimum_required.green {
                        minimum_required.green = play.green;
                    }
                    if play.blue > minimum_required.blue {
                        minimum_required.blue = play.blue;
                    }
                }
                Some(minimum_required.red * minimum_required.green * minimum_required.blue)
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
