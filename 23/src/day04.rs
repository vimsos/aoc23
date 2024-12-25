mod aoc;

use aoc::Solver;
use itertools::Itertools;

fn main() {
    ("day04.txt", [part_1, part_2]).solve();
}

fn part_1(input: &str) -> String {
    parse_cards(input)
        .iter()
        .map(Card::points)
        .sum::<usize>()
        .to_string()
}

fn part_2(input: &str) -> String {
    let card_matches = parse_cards(input).iter().map(Card::matches).collect_vec();
    let mut counts = vec![1; card_matches.len()];

    for (i, matches) in card_matches.iter().enumerate() {
        for offset in 1..matches + 1 {
            counts[i + offset] += counts[i];
        }
    }

    counts.iter().sum::<usize>().to_string()
}

struct Card {
    winning: Vec<usize>,
    played: Vec<usize>,
}

impl Card {
    fn matches(&self) -> usize {
        self.played
            .iter()
            .filter(|p| self.winning.contains(p))
            .count()
    }

    fn points(&self) -> usize {
        match self.matches() {
            0 => 0,
            matches => 1 << matches - 1,
        }
    }
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(":").unwrap().1.split_once("|").unwrap();

            Card {
                winning: left
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_vec(),
                played: right
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_vec(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_GAME: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part_1_test() {
        assert_eq!("13", part_1(EXAMPLE_GAME));
    }

    #[test]
    fn part_2_test() {
        assert_eq!("30", part_2(EXAMPLE_GAME));
    }
}
