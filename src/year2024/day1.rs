use nom::character::complete::{line_ending, space1, u32 as nom_u32};
use nom::combinator::{all_consuming, opt};
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::{sequence::separated_pair, IResult};
use std::collections::HashMap;

use crate::Cli;

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    all_consuming(terminated(
        separated_list1(line_ending, separated_pair(nom_u32, space1, nom_u32)),
        opt(line_ending),
    ))(input)
}

pub(crate) fn part1(cli: Cli) -> String {
    let input = std::fs::read_to_string(cli.file).unwrap();
    let (_, data) = parse(&input).unwrap();

    let mut left_data = Vec::new();
    let mut right_data = Vec::new();

    for (left_data_item, right_data_item) in data {
        left_data.push(left_data_item);
        right_data.push(right_data_item);
    }

    left_data.sort_unstable();
    right_data.sort_unstable();

    let mut total = 0;

    for (left_data_item, right_data_item) in left_data.into_iter().zip(right_data) {
        total += left_data_item.abs_diff(right_data_item);
    }

    format!("part1: {}", total)
}

pub(crate) fn part2(cli: Cli) -> String {
    let input = std::fs::read_to_string(cli.file).unwrap();
    let (_, data) = parse(&input).unwrap();

    let mut left_data: HashMap<u32, u32> = HashMap::new();
    let mut right_data: HashMap<u32, u32> = HashMap::new();

    for (left_data_item, right_data_item) in data {
        *left_data.entry(left_data_item).or_default() += 1;
        *right_data.entry(right_data_item).or_default() += 1;
    }

    let mut total = 0;

    for (left_data_item, left_data_item_cont) in left_data.into_iter() {
        total += left_data_item
            * left_data_item_cont
            * right_data
                .get(&left_data_item)
                .map(|x| *x)
                .unwrap_or_default();
    }

    format!("part2: {}", total)
}
