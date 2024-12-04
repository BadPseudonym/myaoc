use crate::Cli;
use nom::{
    character::complete::{line_ending, space1, u32 as nom_u32},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    all_consuming(terminated(
        separated_list1(line_ending, separated_list1(space1, nom_u32)),
        opt(line_ending),
    ))(input)
}

fn is_safe(val1: u32, val2: u32, increasing: &mut Option<bool>) -> bool {
    if val1 == val2 || val1.abs_diff(val2) > 3 {
        return false;
    }

    if val1 < val2 {
        if *increasing == Some(false) {
            return false;
        }

        *increasing = Some(true);
    } else if val1 > val2 {
        if *increasing == Some(true) {
            return false;
        }

        *increasing = Some(false);
    }

    true
}

fn is_report_safe(report: &[u32]) -> bool {
    let mut increasing = None;

    for level in report.windows(2) {
        let [first, second] = level.try_into().unwrap();

        if !is_safe(first, second, &mut increasing) {
            return false;
        }
    }

    true
}

pub(crate) fn part1(cli: Cli) -> String {
    let input = std::fs::read_to_string(cli.file).unwrap();
    let (_, data) = parse(&input).unwrap();

    let mut safe_count = 0;

    for report in data {
        if is_report_safe(&report) {
            safe_count += 1;
        }
    }

    format!("part1: {}", safe_count)
}

//TODO: make a version thats not like O(N**2) but O(N)
pub(crate) fn part2(cli: Cli) -> String {
    let input = std::fs::read_to_string(cli.file).unwrap();
    let (_, data) = parse(&input).unwrap();

    let mut safe_count = 0;

    'outer: for report in data {
        if is_report_safe(&report) {
            safe_count += 1;
            continue 'outer;
        }

        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);
            if is_report_safe(&report) {
                safe_count += 1;
                continue 'outer;
            }
        }
    }

    format!("part2: {}", safe_count)
}
