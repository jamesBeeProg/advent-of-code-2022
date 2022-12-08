use itertools::Itertools;

use crate::{Solution, Test};

pub struct Day6;

impl Solution for Day6 {
    type Input = Vec<(usize, char)>;
    fn parse(input: &str) -> Self::Input {
        input.trim().char_indices().collect()
    }

    fn part1(input: &Self::Input) -> u32 {
        find_consecutive_unique(input, 4) as u32
    }

    fn part2(input: &Self::Input) -> u32 {
        find_consecutive_unique(input, 14) as u32
    }
}

fn find_consecutive_unique(input: &[(usize, char)], size: usize) -> usize {
    input
        .windows(size)
        .find(|window| window.iter().map(|(_, ch)| ch).unique().count() == size)
        .unwrap()
        .last()
        .unwrap()
        .0
        + 1
}

impl Test for Day6 {
    const TEST_OUTPUT1: u32 = 7;
    const TEST_OUTPUT2: u32 = 19;
    const TEST_INPUT: &'static str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
}
