mod aoc;

use aoc::Solver;

fn main() {
    ("day01.txt", [part_1, part_2]).solve();
}

fn part_1(input: &str) -> String {
    solve_with_extractor(input, extract_numbers_1)
}

fn part_2(input: &str) -> String {
    solve_with_extractor(input, extract_numbers_2)
}

fn solve_with_extractor(input: &str, extractor: fn(&str) -> Vec<u32>) -> String {
    input
        .lines()
        .map(extractor)
        .map(|numbers| numbers.first().unwrap() * 10 + numbers.last().unwrap())
        .sum::<u32>()
        .to_string()
}

fn extract_numbers_1(line: &str) -> Vec<u32> {
    line.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn extract_numbers_2(line: &str) -> Vec<u32> {
    let mut numbers = Vec::<u32>::new();

    let valid_numbers = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for offset in 0..line.len() {
        let rest = &line[offset..];
        if let Some(number) = rest.chars().nth(0).and_then(|c| c.to_digit(10)) {
            numbers.push(number);
            continue;
        } else {
            for possible_number in valid_numbers {
                if rest.starts_with(possible_number.0) {
                    numbers.push(possible_number.1)
                }
            }
        }
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1abc2", vec![1,2])]
    #[case("pqr3stu8vwx", vec![3,8])]
    #[case("a1b2c3d4e5f", vec![1,2,3,4,5])]
    #[case("treb7uchet", vec![7])]
    fn extract_numbers_1_test(#[case] input: &str, #[case] expected: Vec<u32>) {
        assert_eq!(expected, extract_numbers_1(input))
    }

    #[rstest]
    #[case("two1nine", vec![2,1,9])]
    #[case("eightwothree", vec![8,2,3])]
    #[case("abcone2threexyz", vec![1,2,3])]
    #[case("xtwone3four", vec![2,1,3,4])]
    #[case("4nineeightseven2", vec![4,9,8,7,2])]
    #[case("zoneight234", vec![1,8,2,3,4])]
    #[case("7pqrstsixteen", vec![7,6])]
    fn extract_numbers_2_test(#[case] input: &str, #[case] expected: Vec<u32>) {
        assert_eq!(expected, extract_numbers_2(input))
    }
}
