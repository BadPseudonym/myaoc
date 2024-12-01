use crate::{Cli, Part::*};

mod day1;

pub(crate) fn year2024(cli: Cli) -> String {
    match (cli.day, cli.part) {
        (1, Part1) => day1::part1(cli),
        (1, Part2) => day1::part2(cli),
        (day, _) => unreachable!("Unknown day {}", day),
    }
}
