use nom::{
    character::complete::{line_ending, one_of},
    combinator::{all_consuming, opt},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

use crate::Cli;

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    all_consuming(terminated(
        separated_list1(line_ending, many1(one_of("XMAS"))),
        opt(line_ending),
    ))(input)
}

fn offset_2d_get<'a, T>(
    input: &'a Vec<Vec<T>>,
    x: usize,
    y: usize,
    x_offset: isize,
    y_offset: isize,
) -> Option<&'a T> {
    x.checked_add_signed(x_offset)
        .zip(y.checked_add_signed(y_offset))
        .and_then(|(x, y)| input.get(y).and_then(|x_row| x_row.get(x)))
}

pub(crate) fn part1(cli: Cli) -> String {
    let input = std::fs::read_to_string(cli.file).unwrap();
    let (_, input) = parse(&input).unwrap();

    let mut total = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] != 'X' {
                continue;
            }

            'offset_loop: for offset in [
                (0, 1),
                (1, 0),
                (0, -1),
                (-1, 0),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ] {
                for (i, check) in "XMAS".chars().enumerate().skip(1) {
                    let row_char =
                        offset_2d_get(&input, x, y, i as isize * offset.0, i as isize * offset.1);

                    if row_char != Some(&check) {
                        continue 'offset_loop;
                    }
                }
                total += 1;
            }
        }
    }

    format!("part1: {}", total)
}

pub(crate) fn part2(cli: Cli) -> String {
    let input = std::fs::read_to_string(cli.file).unwrap();
    let (_, input) = parse(&input).unwrap();

    let mut total = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] != 'A' {
                continue;
            }

            let left_up = offset_2d_get(&input, x, y, -1, 1);
            let right_down = offset_2d_get(&input, x, y, 1, -1);

            let right_up = offset_2d_get(&input, x, y, 1, 1);
            let left_down = offset_2d_get(&input, x, y, -1, -1);

            match (left_up, right_down) {
                (Some('M'), Some('S')) | (Some('S'), Some('M')) => (),
                _ => continue,
            }

            match (right_up, left_down) {
                (Some('M'), Some('S')) | (Some('S'), Some('M')) => (),
                _ => continue,
            }

            total += 1;
        }
    }

    format!("part2: {}", total)
}
