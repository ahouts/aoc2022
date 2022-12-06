use std::io::Read;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1, newline, space1, u8},
    multi::{many1, many_m_n},
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

enum CrateSpot {
    Empty,
    Crate(char),
}

fn crate_spot(input: &str) -> IResult<&str, CrateSpot> {
    alt((
        terminated(
            delimited(tag("["), anychar, tag("]")),
            many_m_n(0, 1, tag(" ")),
        )
        .map(CrateSpot::Crate),
        many_m_n(3, 4, tag(" ")).map(|_| CrateSpot::Empty),
    ))(input)
}

struct CrateRow {
    spots: Vec<CrateSpot>,
}

fn crate_row(input: &str) -> IResult<&str, CrateRow> {
    many1(crate_spot)
        .map(|spots| CrateRow { spots })
        .parse(input)
}

struct MoveCommand {
    count: u8,
    src: u8,
    dest: u8,
}

fn move_command(input: &str) -> IResult<&str, MoveCommand> {
    tuple((
        preceded(tag("move "), u8),
        preceded(tag(" from "), u8),
        preceded(tag(" to "), u8),
    ))
    .map(|(count, src, dest)| MoveCommand { count, src, dest })
    .parse(input)
}

struct Instructions {
    rows: Vec<CrateRow>,
    commands: Vec<MoveCommand>,
}

fn instructions(input: &str) -> IResult<&str, Instructions> {
    tuple((
        many1(terminated(crate_row, newline)),
        many1(alt((digit1, space1))),
        newline,
        newline,
        many1(terminated(move_command, newline)),
    ))
    .map(|(rows, _, _, _, commands)| Instructions { rows, commands })
    .parse(input)
}

fn part1() -> String {
    let mut text = String::new();
    std::io::BufReader::new(std::fs::File::open("day5/input.txt").unwrap())
        .read_to_string(&mut text)
        .unwrap();
    let (_, Instructions { rows, commands }) = instructions(text.as_str()).unwrap();
    let row_count = rows.iter().map(|row| row.spots.len()).max().unwrap();

    let mut data: Vec<Vec<char>> = (0..row_count).map(|_| Vec::new()).collect();
    for row in rows.iter().rev() {
        for (cell, dcell) in row.spots.iter().zip(data.iter_mut()) {
            if let CrateSpot::Crate(c) = cell {
                dcell.push(*c);
            }
        }
    }

    for command in commands {
        for _ in 0..command.count {
            let c = data[command.src as usize - 1].pop().unwrap();
            data[command.dest as usize - 1].push(c);
        }
    }

    data.into_iter()
        .map(|drow| drow.last().copied().map(String::from).unwrap_or_default())
        .collect()
}

fn part2() -> String {
    let mut text = String::new();
    std::io::BufReader::new(std::fs::File::open("day5/input.txt").unwrap())
        .read_to_string(&mut text)
        .unwrap();
    let (_, Instructions { rows, commands }) = instructions(text.as_str()).unwrap();
    let row_count = rows.iter().map(|row| row.spots.len()).max().unwrap();

    let mut data: Vec<Vec<char>> = (0..row_count).map(|_| Vec::new()).collect();
    for row in rows.iter().rev() {
        for (cell, dcell) in row.spots.iter().zip(data.iter_mut()) {
            if let CrateSpot::Crate(c) = cell {
                dcell.push(*c);
            }
        }
    }

    for command in commands {
        let mut to_add = Vec::new();
        for _ in 0..command.count {
            to_add.push(data[command.src as usize - 1].pop().unwrap());
        }
        for c in to_add.into_iter().rev() {
            data[command.dest as usize - 1].push(c);
        }
    }

    data.into_iter()
        .map(|drow| drow.last().copied().map(String::from).unwrap_or_default())
        .collect()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
