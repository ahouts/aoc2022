use std::{collections::BTreeSet, io::BufRead};

fn part1() -> usize {
    fn parse_part(part: &str) -> (i32, i32) {
        let (x, y) = part.split_once(",").unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }

    let mut grid = BTreeSet::<(i32, i32)>::new();

    std::io::BufReader::new(std::fs::File::open("day14/input.txt").unwrap())
        .lines()
        .for_each(|result| {
            let line = result.unwrap();
            let mut parts = line.split(" -> ");
            let (mut sx, mut sy) = parse_part(parts.next().unwrap());
            for (ex, ey) in parts.map(parse_part) {
                if sx == ex {
                    for dy in i32::min(sy, ey)..=i32::max(sy, ey) {
                        grid.insert((sx, dy));
                    }
                    sy = ey;
                } else {
                    for dx in i32::min(sx, ex)..=i32::max(sx, ex) {
                        grid.insert((dx, sy));
                    }
                    sx = ex;
                }
            }
        });

    let abyss_y = grid.iter().map(|(_, y)| *y).max().unwrap() + 1;

    let mut sand_produced = 0;
    let mut sx = 500;
    let mut sy = 0;
    while sy <= abyss_y {
        if !grid.contains(&(sx, sy + 1)) {
            sy += 1;
            continue;
        }
        if !grid.contains(&(sx - 1, sy + 1)) {
            sx -= 1;
            sy += 1;
            continue;
        }
        if !grid.contains(&(sx + 1, sy + 1)) {
            sx += 1;
            sy += 1;
            continue;
        }
        grid.insert((sx, sy));
        sand_produced += 1;
        sx = 500;
        sy = 0;
    }

    sand_produced
}

fn part2() -> usize {
    fn parse_part(part: &str) -> (i32, i32) {
        let (x, y) = part.split_once(",").unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }

    let mut grid = BTreeSet::<(i32, i32)>::new();

    std::io::BufReader::new(std::fs::File::open("day14/input.txt").unwrap())
        .lines()
        .for_each(|result| {
            let line = result.unwrap();
            let mut parts = line.split(" -> ");
            let (mut sx, mut sy) = parse_part(parts.next().unwrap());
            for (ex, ey) in parts.map(parse_part) {
                if sx == ex {
                    for dy in i32::min(sy, ey)..=i32::max(sy, ey) {
                        grid.insert((sx, dy));
                    }
                    sy = ey;
                } else {
                    for dx in i32::min(sx, ex)..=i32::max(sx, ex) {
                        grid.insert((dx, sy));
                    }
                    sx = ex;
                }
            }
        });

    let floor_y = grid.iter().map(|(_, y)| *y).max().unwrap() + 1;

    let mut sand_produced = 0;
    let mut sx = 500;
    let mut sy = 0;
    loop {
        if sx == 500 && sy == 0 && grid.contains(&(sx, sy)) {
            break;
        }

        if !grid.contains(&(sx, sy + 1)) && floor_y != sy {
            sy += 1;
            continue;
        }
        if !grid.contains(&(sx - 1, sy + 1)) && floor_y != sy {
            sx -= 1;
            sy += 1;
            continue;
        }
        if !grid.contains(&(sx + 1, sy + 1)) && floor_y != sy {
            sx += 1;
            sy += 1;
            continue;
        }
        grid.insert((sx, sy));
        sand_produced += 1;
        sx = 500;
        sy = 0;
    }

    sand_produced
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
