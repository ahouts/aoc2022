use std::{collections::BTreeSet, io::BufRead};

fn part1() -> usize {
    std::io::BufReader::new(std::fs::File::open("day3/input.txt").unwrap())
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap()
        .into_iter()
        .flat_map(|line| {
            let (a, b) = line.trim().split_at(line.len() / 2);
            a.chars()
                .collect::<BTreeSet<char>>()
                .intersection(&b.chars().collect::<BTreeSet<char>>())
                .next()
                .copied()
        })
        .map(|c| {
            if c.is_lowercase() {
                c as usize - 'a' as usize + 1
            } else {
                c as usize - 'A' as usize + 27
            }
        })
        .sum()
}

fn part2() -> usize {
    std::io::BufReader::new(std::fs::File::open("day3/input.txt").unwrap())
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap()
        .chunks_exact(3)
        .into_iter()
        .flat_map(|lines| match lines {
            [l1, l2, l3] => l1
                .chars()
                .collect::<BTreeSet<char>>()
                .intersection(&l2.chars().collect::<BTreeSet<char>>())
                .copied()
                .collect::<BTreeSet<char>>()
                .intersection(&l3.chars().collect::<BTreeSet<char>>())
                .into_iter()
                .next()
                .copied(),
            _ => panic!(),
        })
        .map(|c| {
            if c.is_lowercase() {
                c as usize - 'a' as usize + 1
            } else {
                c as usize - 'A' as usize + 27
            }
        })
        .sum()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
