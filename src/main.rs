pub mod year2024;

use std::path::PathBuf;

use clap::{Parser, ValueEnum};

fn main() {
    let cli = Cli::parse();

    let result = match cli.year {
        2024 => year2024::year2024(cli),
        year => unreachable!("Unknown year {}", year),
    };

    println!("Result: {}", result);
}

#[derive(Parser, Clone, Debug)]
pub(crate) struct Cli {
    year: usize,

    day: usize,

    part: Part,

    file: PathBuf,
}

#[derive(ValueEnum, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) enum Part {
    Part1,
    Part2,
}
