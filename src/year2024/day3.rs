use crate::Cli;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, u32 as nom_u32},
    combinator::map,
    error,
    multi::{many1, many_till},
    sequence::tuple,
};

//TODO: redo all this to make it more efficiently and readable

pub(crate) fn part1(cli: Cli) -> String {
    let input = std::fs::read_to_string(cli.file).unwrap();

    let (_, result) = many1::<_, _, error::Error<_>, _>(many_till(
        anychar,
        tuple((tag("mul("), nom_u32, tag(","), nom_u32, tag(")"))),
    ))(input.as_str())
    .unwrap();

    let mut total = 0;

    for (_, (_, val1, _, val2, _)) in result {
        total += val1 * val2;
    }

    format!("part1: {}", total)
}

enum Instruction {
    Mul(u32, u32),
    r#Do,
    Dont,
}

pub(crate) fn part2(cli: Cli) -> String {
    let input = std::fs::read_to_string(cli.file).unwrap();

    let (_, result) = many1::<_, _, error::Error<_>, _>(many_till(
        anychar,
        alt((
            map(
                tuple((tag("mul("), nom_u32, tag(","), nom_u32, tag(")"))),
                |(_, val1, _, val2, _)| Instruction::Mul(val1, val2),
            ),
            map(tag("do()"), |_| Instruction::r#Do),
            map(tag("don't()"), |_| Instruction::Dont),
        )),
    ))(input.as_str())
    .unwrap();

    let mut total = 0;

    let mut enabled = true;

    for (_, instruction) in result {
        match instruction {
            Instruction::r#Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Mul(val1, val2) => {
                if enabled {
                    total += val1 * val2;
                }
            }
        }
    }

    format!("part2: {}", total)
}
