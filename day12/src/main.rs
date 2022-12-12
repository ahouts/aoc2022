use std::{collections::BTreeMap, io::BufRead};

fn get_grid() -> Vec<Vec<u8>> {
    std::io::BufReader::new(std::fs::File::open("day12/input.txt").unwrap())
        .lines()
        .map(|result| {
            result
                .unwrap()
                .chars()
                .map(|c| match c {
                    'S' => 254,
                    'E' => 255,
                    _ => c as u8 - b'a',
                })
                .collect()
        })
        .collect()
}

fn find_shortest_path(
    grid: &Vec<Vec<u8>>,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
) -> usize {
    let mut dist_to_pos = Vec::new();
    let mut pos_to_dist = BTreeMap::new();

    dist_to_pos.push((0, (start_x as isize, start_y as isize)));
    pos_to_dist.insert((start_x as isize, start_y as isize), 0);

    loop {
        if let Some(dist) = pos_to_dist.get(&(end_x as isize, end_y as isize)) {
            return *dist;
        }

        dist_to_pos.sort_by_key(|(dist, _)| *dist);
        dist_to_pos.reverse();

        if let Some((_, (x, y))) = dist_to_pos.pop() {
            let value = grid[x as usize][y as usize];
            for (xi, yi) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let xx = x + xi;
                let yy = y + yi;

                if pos_to_dist.contains_key(&(xx, yy)) {
                    continue;
                }

                if xx < 0 || xx >= grid.len() as isize || yy < 0 || yy >= grid[0].len() as isize {
                    continue;
                }

                if (grid[xx as usize][yy as usize] as i8 - value as i8) > 1 {
                    continue;
                }

                dist_to_pos.push((pos_to_dist[&(x, y)] + 1, (xx, yy)));
                pos_to_dist.insert((xx, yy), pos_to_dist[&(x, y)] + 1);
            }
        } else {
            return usize::MAX;
        }
    }
}

fn part1() -> usize {
    let mut grid = get_grid();

    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == 254 {
                start_x = x;
                start_y = y;
            }
            if grid[x][y] == 255 {
                end_x = x;
                end_y = y;
            }
        }
    }
    grid[start_x][start_y] = 0;
    grid[end_x][end_y] = b'z' - b'a';

    find_shortest_path(&grid, start_x, start_y, end_x, end_y)
}

fn part2() -> usize {
    let mut grid = get_grid();

    let mut end_x = 0;
    let mut end_y = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == 254 {
                grid[x][y] = 0;
            }
            if grid[x][y] == 255 {
                end_x = x;
                end_y = y;
            }
        }
    }
    grid[end_x][end_y] = b'z' - b'a';

    (0..grid.len())
        .flat_map(|x| (0..grid[0].len()).map(move |y| (x, y)))
        .filter(|(x, y)| grid[*x][*y] == 0)
        .map(|(x, y)| find_shortest_path(&grid, x, y, end_x, end_y))
        .min()
        .unwrap()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
