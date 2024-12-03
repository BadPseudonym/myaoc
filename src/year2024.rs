use crate::{Cli, Part::*};

mod day1;
mod day2;
mod day3;

pub(crate) fn year2024(cli: Cli) -> String {
    match (cli.day, cli.part) {
        (1, Part1) => day1::part1(cli),
        (1, Part2) => day1::part2(cli),
        (2, Part1) => day2::part1(cli),
        (2, Part2) => day2::part2(cli),
        (3, Part1) => day3::part1(cli),
        (3, Part2) => day3::part2(cli),
        (day, _) => unreachable!("Unknown day {}", day),
    }
}
