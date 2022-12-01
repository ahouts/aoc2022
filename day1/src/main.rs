use itertools::Itertools;
use std::{io::BufRead, str::FromStr};

fn part1() -> usize {
    std::io::BufReader::new(std::fs::File::open("day1/input.txt").unwrap())
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap()
        .into_iter()
        .map(|line| -> Result<Option<usize>, <usize as FromStr>::Err> {
            if line.is_empty() {
                Ok(None)
            } else {
                Ok(Some(line.parse::<usize>()?))
            }
        })
        .collect::<Result<Vec<Option<usize>>, _>>()
        .unwrap()
        .into_iter()
        .group_by(Option::is_some)
        .into_iter()
        .filter(|(is_some, _)| *is_some)
        .map(|(_, group)| group.map(Option::unwrap))
        .map(|group| group.sum::<usize>())
        .max()
        .unwrap()
}

fn part2() -> usize {
    std::io::BufReader::new(std::fs::File::open("day1/input.txt").unwrap())
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap()
        .into_iter()
        .map(|line| -> Result<Option<usize>, <usize as FromStr>::Err> {
            if line.is_empty() {
                Ok(None)
            } else {
                Ok(Some(line.parse::<usize>()?))
            }
        })
        .collect::<Result<Vec<Option<usize>>, _>>()
        .unwrap()
        .into_iter()
        .group_by(Option::is_some)
        .into_iter()
        .filter(|(is_some, _)| *is_some)
        .map(|(_, group)| group.map(Option::unwrap))
        .map(|group| group.sum::<usize>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
