use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u32 as nom_u32},
    combinator::{all_consuming, opt},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::Cli;

fn parse(input: &str) -> IResult<&str, (Vec<(u32, u32)>, Vec<Vec<u32>>)> {
    all_consuming(terminated(
        separated_pair(
            many1(terminated(
                separated_pair(nom_u32, tag("|"), nom_u32),
                line_ending,
            )),
            line_ending,
            separated_list1(line_ending, separated_list1(tag(","), nom_u32)),
        ),
        opt(line_ending),
    ))(input)
}

pub(crate) fn part1(cli: Cli) -> String {
    let input = std::fs::read_to_string(cli.file).unwrap();
    let (_, (page_ordering_rules, updates)) = parse(&input).unwrap();

    let mut cached_page_ordering_rules: HashMap<u32, HashSet<u32>> = HashMap::new();

    for (page_ordering_rule_before, page_ordering_rule_after) in page_ordering_rules {
        cached_page_ordering_rules
            .entry(page_ordering_rule_before)
            .or_default()
            .insert(page_ordering_rule_after);
    }

    let mut total_middle = 0;

    'updates: for update in updates {
        let mut pages_seen = HashSet::new();

        for update_part in &update {
            if pages_seen
                .intersection(cached_page_ordering_rules.entry(*update_part).or_default())
                .next()
                .is_some()
            {
                continue 'updates;
            }

            pages_seen.insert(*update_part);
        }
        total_middle += update[update.len() / 2];
    }

    format!("part1: {}", total_middle)
}

pub(crate) fn part2(cli: Cli) -> String {
    let input = std::fs::read_to_string(cli.file).unwrap();
    let (_, (page_ordering_rules, updates)) = parse(&input).unwrap();

    let mut cached_page_ordering_rules: HashMap<u32, HashSet<u32>> = HashMap::new();

    for (page_ordering_rule_before, page_ordering_rule_after) in page_ordering_rules {
        cached_page_ordering_rules
            .entry(page_ordering_rule_before)
            .or_default()
            .insert(page_ordering_rule_after);
    }

    let mut total_middle = 0;

    for mut update in updates {
        let mut changed = false;

        let mut pages_seen = HashSet::new();

        let mut update_part_i = 0;

        while {
            let mut clashes: Vec<_> = pages_seen
                .intersection(
                    cached_page_ordering_rules
                        .entry(update[update_part_i])
                        .or_default(),
                )
                .map(|x1| update.iter().position(|x2| x1 == x2).unwrap())
                .collect();

            clashes.sort_unstable();

            if !clashes.is_empty() {
                let to_move = update.remove(update_part_i);

                update.insert(clashes[0], to_move);

                update_part_i = 0;
                pages_seen.drain();

                changed = true;

                println!("{:?}", update);
            }

            pages_seen.insert(update[update_part_i]);

            update_part_i += 1;
            update_part_i != update.len()
        } {}

        if changed {
            total_middle += update[update.len() / 2];
        }
    }

    format!("part1: {}", total_middle)
}
