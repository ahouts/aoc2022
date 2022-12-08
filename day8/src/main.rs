use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part1() -> usize {
    let forest = BufReader::new(File::open("day8/input.txt").unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| String::from(c).parse().unwrap())
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();

    let size = forest.len();
    let mut visibilities = vec![vec![0_i8; size]; size];

    for x in 0..size {
        for y in 0..size {
            let on_edge = x == 0 || y == 0 || x == size - 1 || y == size - 1;
            if on_edge {
                visibilities[x][y] = -1;
            } else {
                let lowest_vis = (0..x)
                    .map(|xi| forest[xi][y])
                    .max()
                    .unwrap_or(10)
                    .min(((x + 1)..size).map(|xi| forest[xi][y]).max().unwrap_or(10))
                    .min((0..y).map(|yi| forest[x][yi]).max().unwrap_or(10))
                    .min(((y + 1)..size).map(|yi| forest[x][yi]).max().unwrap_or(10));
                visibilities[x][y] = lowest_vis;
            }
        }
    }

    let mut visible_count = 0;
    for x in 0..size {
        for y in 0..size {
            if forest[x][y] > visibilities[x][y] {
                visible_count += 1;
            }
        }
    }

    visible_count
}

fn part2() -> usize {
    let forest = BufReader::new(File::open("day8/input.txt").unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| String::from(c).parse().unwrap())
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();

    let size = forest.len();

    let mut max_score = 0;
    for x in 0..size {
        for y in 0..size {
            let h = forest[x][y];

            let mut w_trees = 0;
            for xi in (0..x).rev() {
                w_trees += 1;
                if forest[xi][y] >= h {
                    break;
                }
            }

            let mut n_trees = 0;
            for yi in (0..y).rev() {
                n_trees += 1;
                if forest[x][yi] >= h {
                    break;
                }
            }

            let mut e_trees = 0;
            for xi in (x + 1)..size {
                e_trees += 1;
                if forest[xi][y] >= h {
                    break;
                }
            }

            let mut s_trees = 0;
            for yi in (y + 1)..size {
                s_trees += 1;
                if forest[x][yi] >= h {
                    break;
                }
            }

            let score = w_trees * n_trees * e_trees * s_trees;
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
