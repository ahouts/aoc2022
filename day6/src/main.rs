use std::{collections::BTreeSet, io::Read};

fn part1() -> usize {
    let mut text = String::new();
    std::io::BufReader::new(std::fs::File::open("day6/input.txt").unwrap())
        .read_to_string(&mut text)
        .unwrap();
    text.as_bytes()
        .windows(4)
        .enumerate()
        .filter(|(_, window)| window.iter().copied().collect::<BTreeSet<_>>().len() == 4)
        .map(|(idx, _)| idx + 4)
        .next()
        .unwrap()
}

fn part2() -> usize {
    let mut text = String::new();
    std::io::BufReader::new(std::fs::File::open("day6/input.txt").unwrap())
        .read_to_string(&mut text)
        .unwrap();
    text.as_bytes()
        .windows(14)
        .enumerate()
        .filter(|(_, window)| window.iter().copied().collect::<BTreeSet<_>>().len() == 14)
        .map(|(idx, _)| idx + 14)
        .next()
        .unwrap()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
