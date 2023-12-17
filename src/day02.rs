mod aoc;

use aoc::Solver;

fn main() {
    ("day02.txt", [part_1, part_2]).solve();
}

fn part_1(input: &str) -> String {
    input
        .lines()
        .map(parse_line)
        .filter(check_feasibility)
        .map(|rs| rs.first().unwrap().id)
        .sum::<usize>()
        .to_string()
}

fn part_2(input: &str) -> String {
    input
        .lines()
        .map(parse_line)
        .map(|rs| check_fewest(&rs))
        .map(|r| r.red * r.green * r.blue)
        .sum::<usize>()
        .to_string()
}

fn check_feasibility(rounds: &Vec<Round>) -> bool {
    const RED_MAX: usize = 12;
    const GREEN_MAX: usize = 13;
    const BLUE_MAX: usize = 14;

    rounds
        .iter()
        .all(|r| r.red <= RED_MAX && r.green <= GREEN_MAX && r.blue <= BLUE_MAX)
}

fn check_fewest(rounds: &Vec<Round>) -> Round {
    rounds.iter().fold(Default::default(), |f, e| Round {
        id: 0,
        red: f.red.max(e.red),
        green: f.green.max(e.green),
        blue: f.blue.max(e.blue),
    })
}

fn parse_line(line: &str) -> Vec<Round> {
    let (header, values) = line.split_once(':').unwrap();

    let id = header.split_once(' ').unwrap().1.parse::<usize>().unwrap();

    values
        .split(';')
        .map(|l: &str| -> Round {
            let mut red = 0 as usize;
            let mut green = 0 as usize;
            let mut blue = 0 as usize;

            for (quantity_str, color) in l.split(',').filter_map(|s| s.trim().split_once(' ')) {
                if let Ok(quantity) = quantity_str.parse::<usize>() {
                    match color {
                        "red" => {
                            red = quantity;
                        }
                        "green" => {
                            green = quantity;
                        }
                        "blue" => {
                            blue = quantity;
                        }
                        _ => unreachable!(),
                    }
                } else {
                    unreachable!();
                }
            }

            Round {
                id,
                red,
                green,
                blue,
            }
        })
        .collect()
}

#[derive(Default)]
struct Round {
    id: usize,
    red: usize,
    green: usize,
    blue: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        true
    )]
    #[case(
        "Game 2003: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        false
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        false
    )]
    #[case("Game 15: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]
    fn extract_numbers_1_test(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(expected, check_feasibility(&parse_line(input)))
    }
}
