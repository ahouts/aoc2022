use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn part1() -> usize {
    let mut hx: i32 = 0;
    let mut hy: i32 = 0;
    let mut tx: i32 = 0;
    let mut ty: i32 = 0;
    let mut visited = BTreeSet::new();
    visited.insert((tx, ty));
    for result in BufReader::new(File::open("day9/input.txt").unwrap()).lines() {
        let line = result.unwrap();
        let (dir, dist) = line.split_at(1);
        for _ in 0..(dist.trim().parse::<usize>().unwrap()) {
            match dir {
                "L" => hx -= 1,
                "R" => hx += 1,
                "U" => hy += 1,
                "D" => hy -= 1,
                _ => panic!(),
            }
            if (hx - tx).abs() > 1 || (hy - ty).abs() > 1 {
                match (hx - tx, hy - ty) {
                    (0, 2) => ty += 1,
                    (0, -2) => ty -= 1,
                    (2, 0) => tx += 1,
                    (-2, 0) => tx -= 1,
                    (2, 1) | (1, 2) => {
                        tx += 1;
                        ty += 1;
                    }
                    (-2, 1) | (-1, 2) => {
                        tx -= 1;
                        ty += 1;
                    }
                    (2, -1) | (1, -2) => {
                        tx += 1;
                        ty -= 1;
                    }
                    (-2, -1) | (-1, -2) => {
                        tx -= 1;
                        ty -= 1;
                    }
                    _ => panic!(),
                }
                visited.insert((tx, ty));
            }
        }
    }
    visited.len()
}

fn part2() -> usize {
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut tail_visited = BTreeSet::new();
    tail_visited.insert(knots[9]);
    for result in BufReader::new(File::open("day9/input.txt").unwrap()).lines() {
        let line = result.unwrap();
        let (dir, dist) = line.split_at(1);
        for _ in 0..(dist.trim().parse::<usize>().unwrap()) {
            {
                let (hx, hy) = &mut knots[0];
                match dir {
                    "L" => *hx -= 1,
                    "R" => *hx += 1,
                    "U" => *hy += 1,
                    "D" => *hy -= 1,
                    _ => panic!(),
                }
            }
            for i in 0..(knots.len() - 1) {
                let (temp1, temp2) = knots.split_at_mut(i + 1);
                let (hx, hy) = temp1.last().unwrap();
                let (tx, ty) = temp2.first_mut().unwrap();
                if (*hx - *tx).abs() > 1 || (*hy - *ty).abs() > 1 {
                    match (*hx - *tx, *hy - *ty) {
                        (0, 2) => *ty += 1,
                        (0, -2) => *ty -= 1,
                        (2, 0) => *tx += 1,
                        (-2, 0) => *tx -= 1,
                        (2, 1) | (1, 2) | (2, 2) => {
                            *tx += 1;
                            *ty += 1;
                        }
                        (-2, 1) | (-1, 2) | (-2, 2) => {
                            *tx -= 1;
                            *ty += 1;
                        }
                        (2, -1) | (1, -2) | (2, -2) => {
                            *tx += 1;
                            *ty -= 1;
                        }
                        (-2, -1) | (-1, -2) | (-2, -2) => {
                            *tx -= 1;
                            *ty -= 1;
                        }
                        delta => panic!("unexpected delta {delta:?}"),
                    }
                    if i == 8 {
                        tail_visited.insert(knots[9]);
                    }
                }
            }
        }
    }
    tail_visited.len()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
