mod aoc;
use aoc::Solver;
use itertools::Itertools;
use std::{num::Wrapping, ops::Not};

fn main() {
    ("day03.txt", [part_1, part_2]).solve();
}

fn part_1(input: &str) -> String {
    const SYMBOLS: [char; 10] = ['+', '%', '*', '$', '#', '/', '=', '@', '-', '&'];

    let line_length = input.find('\n').unwrap() + 1;
    let input: Vec<char> = input.chars().collect();

    extract_numbers(&input)
        .iter()
        .filter_map(|n| {
            extract_adjacent_symbol_indexes(&input, *n, line_length, &SYMBOLS)
                .is_empty()
                .not()
                .then_some(n.value.0)
        })
        .sum::<usize>()
        .to_string()
}

fn part_2(input: &str) -> String {
    let line_length = input.find('\n').unwrap() + 1;
    let input: Vec<char> = input.chars().collect();
    let numbers = extract_numbers(&input);

    let gear_number_map = numbers
        .iter()
        .map(|n| {
            extract_adjacent_symbol_indexes(&input, *n, line_length, &['*'])
                .into_iter()
                .map(|i| (i, *n))
        })
        .flatten()
        .into_group_map();

    gear_number_map
        .values()
        .filter(|ns| ns.len() == 2)
        .map(|ns| ns[0].value.0 * ns[1].value.0)
        .sum::<usize>()
        .to_string()
}

#[derive(Copy, Clone)]
struct Number {
    index: Wrapping<usize>,
    length: Wrapping<usize>,
    value: Wrapping<usize>,
}

fn extract_numbers(input: &[char]) -> Vec<Number> {
    input
        .iter()
        .enumerate()
        .fold(
            (Vec::new(), None),
            |(mut numbers, mut previous_number), (i, c)| {
                previous_number = match (c.is_digit(10), previous_number) {
                    (true, None) => Number {
                        index: Wrapping(i),
                        length: Wrapping(1),
                        value: Wrapping(c.to_digit(10).unwrap() as usize),
                    }
                    .into(),
                    (true, Some(n)) if i + 1 == input.len() => {
                        numbers.push(Number {
                            length: n.length + Wrapping(1),
                            value: n.value * Wrapping(10)
                                + Wrapping(c.to_digit(10).unwrap() as usize),
                            ..n
                        });
                        None
                    }
                    (true, Some(n)) => Number {
                        length: n.length + Wrapping(1),
                        value: n.value * Wrapping(10) + Wrapping(c.to_digit(10).unwrap() as usize),
                        ..n
                    }
                    .into(),
                    (false, Some(n)) => {
                        numbers.push(n);
                        None
                    }
                    (false, None) => None,
                };

                (numbers, previous_number)
            },
        )
        .0
}

fn extract_adjacent_symbol_indexes(
    input: &Vec<char>,
    number: Number,
    line_length: usize,
    symbols: &[char],
) -> Vec<usize> {
    let line_length = Wrapping(line_length);

    let above = (number.index - line_length - Wrapping(1)).0
        ..(number.index + number.length - line_length + Wrapping(1)).0;
    let around = (number.index - Wrapping(1)).0..(number.index + number.length + Wrapping(1)).0;
    let below = (number.index + line_length - Wrapping(1)).0
        ..(number.index + number.length + line_length + Wrapping(1)).0;

    above
        .chain(around)
        .chain(below)
        .filter_map(|index| {
            input
                .get(index)
                .is_some_and(|c| symbols.contains(c))
                .then(|| index)
        })
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const EXAMPLE_SCHEMATIC: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    const TEST_SCHEMATIC: &str = ".....
..515
.....
.....
.....";

    const TEST_SCHEMATIC_2: &str = "......
628...
......
505+..
......
......";

    const TEST_SCHEMATIC_3: &str = "....
2.2.
.*..
1.1.";

    const TEST_SCHEMATIC_4: &str = ".......5......
..7*..*.....4*
...*13*......9
.......15.....
..............
..............
..............
..............
..............
..............
21............
...*9.........";

    const TEST_SCHEMATIC_5: &str = "100
200";

    const TEST_SCHEMATIC_6: &str = "2.2.....$123
............";

    const TEST_SCHEMATIC_7: &str = "......
...628
......
*0005.
......
......";

    #[rstest]
    #[case(EXAMPLE_SCHEMATIC, [467,114,35,633,617,58,592,755,664,598].into(), [0, 5, 24, 28, 44, 62, 68, 83, 100, 104].into(), [3, 3, 2, 3, 3, 2, 3, 3, 3, 3].into())]
    #[case(TEST_SCHEMATIC, [515].into(), [8].into(), [3].into())]
    #[case(TEST_SCHEMATIC_2, [628, 505].into(), [7, 21].into(), [3, 3].into())]
    #[case(TEST_SCHEMATIC_3, [2, 2, 1, 1].into(), [5, 7, 15, 17].into(), [1, 1, 1, 1].into())]
    #[case(TEST_SCHEMATIC_4, [5, 7, 4, 13, 9, 15, 21, 9].into(), [7, 17, 27, 34, 43, 52, 150, 169].into(), [1, 1, 1, 2, 1, 2, 2, 1].into())]
    #[case(TEST_SCHEMATIC_5, [100, 200].into(), [0, 4].into(), [3, 3].into())]
    #[case(TEST_SCHEMATIC_6, [2, 2, 123].into(), [0, 2, 9].into(), [1, 1, 3].into())]
    #[case(TEST_SCHEMATIC_7, [628, 5].into(), [10,22].into(), [3, 4].into())]
    fn extract_numbers_test(
        #[case] input: &str,
        #[case] expected_values: Vec<usize>,
        #[case] expected_indexes: Vec<usize>,
        #[case] expected_lenghts: Vec<usize>,
    ) {
        let input: Vec<char> = input.chars().collect();

        assert_eq!(
            extract_numbers(&input)
                .iter()
                .map(|n| n.value.0)
                .collect::<Vec<usize>>(),
            expected_values
        );

        assert_eq!(
            extract_numbers(&input)
                .iter()
                .map(|n| n.index.0)
                .collect::<Vec<usize>>(),
            expected_indexes
        );

        assert_eq!(
            extract_numbers(&input)
                .iter()
                .map(|n| n.length.0)
                .collect::<Vec<usize>>(),
            expected_lenghts
        );
    }

    #[rstest]
    #[case(EXAMPLE_SCHEMATIC, "4361")]
    #[case(TEST_SCHEMATIC, "0")]
    #[case(TEST_SCHEMATIC_2, "505")]
    #[case(TEST_SCHEMATIC_3, "6")]
    #[case(TEST_SCHEMATIC_4, "62")]
    #[case(TEST_SCHEMATIC_5, "0")]
    #[case(TEST_SCHEMATIC_6, "123")]
    #[case(TEST_SCHEMATIC_7, "5")]
    fn part_1_test(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(part_1(input), expected);
    }

    #[rstest]
    #[case(EXAMPLE_SCHEMATIC, "467835")]
    fn part_2_test(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(part_2(input), expected);
    }
}
