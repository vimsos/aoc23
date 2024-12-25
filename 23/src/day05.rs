mod aoc;

use aoc::Solver;
use itertools::Itertools;

fn main() {
    ("day05.txt", [part_1 as fn(&str) -> String, part_2]).solve();
}

fn part_1(input: &str) -> String {
    solve_with(input, Almanac::part_1_seed_parser)
}

fn part_2(input: &str) -> String {
    solve_with(input, Almanac::part_2_seed_parser)
}

fn solve_with(input: &str, seed_parser: fn(&str) -> Vec<Range>) -> String {
    Almanac::from_input(input, seed_parser)
        .map_seeds()
        .iter()
        .map(|m| m.start)
        .min()
        .unwrap()
        .to_string()
}

struct Almanac {
    seeds: Vec<Range>,
    conversion_stages: Vec<Vec<Map>>,
}

struct Map {
    source: Range,
    destination: Range,
}

impl Map {
    fn translate(&self, range: &Range) -> MaybeOverlap {
        let maybe_overlap = self.source.test_overlap(range);

        let mapped_range = maybe_overlap.overlap.and_then(|overlap| {
            let start = overlap.start - self.source.start + self.destination.start;
            let end = start + overlap.len;
            let len = end - start;

            Some(Range::new(start, len))
        });

        MaybeOverlap {
            left: maybe_overlap.left,
            overlap: mapped_range,
            right: maybe_overlap.right,
        }
    }
}

#[derive(PartialEq, Debug)]
struct MaybeOverlap {
    left: Option<Range>,
    overlap: Option<Range>,
    right: Option<Range>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Range {
    start: usize,
    end: usize,
    len: usize,
}

impl Range {
    fn new(start: usize, len: usize) -> Self {
        Self {
            start,
            end: start + len,
            len,
        }
    }

    fn try_from(start: usize, end: usize) -> Option<Self> {
        match start < end {
            true => Some(Self {
                start,
                end,
                len: end - start,
            }),
            false => None,
        }
    }

    fn test_overlap(&self, rhs: &Range) -> MaybeOverlap {
        let left = Range::try_from(self.start.min(rhs.start), self.start.min(rhs.end));
        let overlap = Range::try_from(self.start.max(rhs.start), self.end.min(rhs.end));
        let right = Range::try_from(self.end.max(rhs.start), self.end.max(rhs.end));

        MaybeOverlap {
            left,
            overlap,
            right,
        }
    }
}

impl Almanac {
    fn part_1_seed_parser(line: &str) -> Vec<Range> {
        line.split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .map(|s| Range::new(s, 1))
            .collect()
    }

    fn part_2_seed_parser(line: &str) -> Vec<Range> {
        line.split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .chunks(2)
            .into_iter()
            .map(|ns| ns.collect_vec())
            .map(|ns| Range::new(ns[0], ns[1]))
            .collect()
    }

    fn from_input(input: &str, seed_parser: fn(&str) -> Vec<Range>) -> Self {
        let seeds = seed_parser(input.lines().nth(0).unwrap().split_once(":").unwrap().1);

        let conversion_stages = input
            .split("\n\n")
            .skip(1)
            .map(|m| {
                m.lines()
                    .skip(1)
                    .map(|l| {
                        l.split_whitespace()
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect_vec()
                    })
                    .map(|ns| Map {
                        source: Range::new(ns[1], ns[2]),
                        destination: Range::new(ns[0], ns[2]),
                    })
                    .collect_vec()
            })
            .collect_vec();

        Self {
            seeds,
            conversion_stages,
        }
    }

    fn map_seeds(&self) -> Vec<Range> {
        self.seeds
            .iter()
            .map(|s| {
                let mut seed_ranges = Vec::from_iter([*s]);

                for stage in self.conversion_stages.iter() {
                    let mut stage_output = Vec::new();

                    'seed: while let Some(seed) = seed_ranges.pop() {
                        for map in stage.iter() {
                            let maybe_translated = map.translate(&seed);

                            if let Some(translated) = maybe_translated.overlap {
                                stage_output.push(translated);

                                if let Some(left) = maybe_translated.left {
                                    seed_ranges.push(left);
                                }

                                if let Some(right) = maybe_translated.right {
                                    stage_output.push(right);
                                }

                                continue 'seed;
                            }
                        }

                        stage_output.push(seed);
                    }

                    seed_ranges.append(&mut stage_output);
                }

                seed_ranges
            })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part_1_test() {
        assert_eq!("35", part_1(EXAMPLE));
    }

    #[test]
    fn part_2_test() {
        assert_eq!("46", part_2(EXAMPLE));
    }

    #[rstest]
    #[case(
        Range::new(0, 5),
        Range::new(0, 5),
        MaybeOverlap {
            left: None,
            overlap: Range::try_from(0, 5),
            right: None,
        }
    )]
    #[case(
        Range::new(1, 3),
        Range::new(0, 5),
        MaybeOverlap {
            left: Range::try_from(0, 1),
            overlap: Range::try_from(1, 4),
            right: Range::try_from(4, 5),
        }
    )]
    #[case(
        Range::new(0, 5),
        Range::new(1, 3),
        MaybeOverlap {
            left: None,
            overlap: Range::try_from(1, 4),
            right: None,
        }
    )]
    #[case(
        Range::new(1, 3),
        Range::new(5, 5),
        MaybeOverlap {
            left: None,
            overlap: None,
            right: Range::try_from(5, 10),
        }
    )]
    #[case(
        Range::new(5, 5),
        Range::new(1, 3),
        MaybeOverlap {
            left: Range::try_from(1, 4),
            overlap: None,
            right: None,
        }
    )]
    #[case(
        Range::new(5, 5),
        Range::new(10, 3),
        MaybeOverlap {
            left: None,
            overlap: None,
            right: Range::try_from(10, 13),
        }
    )]
    #[case(
        Range::new(5, 5),
        Range::new(9, 4),
        MaybeOverlap {
            left: None,
            overlap: Range::try_from(9, 10),
            right: Range::try_from(10, 13),
        }
    )]
    #[case(
        Range::new(0, 5),
        Range::new(3, 7),
        MaybeOverlap {
            left: None,
            overlap: Range::try_from(3, 5),
            right: Range::try_from(5, 10),
        }
    )]
    #[case(
        Range::new(3, 7),
        Range::new(0, 5),
        MaybeOverlap {
            left: Range::try_from(0, 3),
            overlap: Range::try_from(3, 5),
            right: None,
        }
    )]
    fn overlap_test(#[case] lhs: Range, #[case] rhs: Range, #[case] expected: MaybeOverlap) {
        assert_eq!(lhs.test_overlap(&rhs), expected);
    }
}
