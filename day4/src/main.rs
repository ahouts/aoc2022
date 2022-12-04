use std::io::BufRead;

fn part1() -> usize {
    std::io::BufReader::new(std::fs::File::open("day4/input.txt").unwrap())
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap()
        .into_iter()
        .filter(|line| {
            let (t1, t2) = line.split_at(line.find(',').unwrap());
            fn parse_range(text: &str) -> (usize, usize) {
                let mut i = text.split('-');
                let l = i.next().unwrap().parse().unwrap();
                let u = i.next().unwrap().parse().unwrap();
                (l, u)
            }
            let r1 = parse_range(t1);
            let r2 = parse_range(&t2[1..]);
            fn contains(r1: (usize, usize), r2: (usize, usize)) -> bool {
                (r1.0..=r1.1).contains(&r2.0) && (r1.0..=r1.1).contains(&r2.1)
            }
            contains(r1, r2) || contains(r2, r1)
        })
        .count()
}

fn part2() -> usize {
    std::io::BufReader::new(std::fs::File::open("day4/input.txt").unwrap())
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap()
        .into_iter()
        .filter(|line| {
            let (t1, t2) = line.split_at(line.find(',').unwrap());
            fn parse_range(text: &str) -> (usize, usize) {
                let mut i = text.split('-');
                let l = i.next().unwrap().parse().unwrap();
                let u = i.next().unwrap().parse().unwrap();
                (l, u)
            }
            let r1 = parse_range(t1);
            let r2 = parse_range(&t2[1..]);
            fn contains(r1: (usize, usize), r2: (usize, usize)) -> bool {
                (r1.0..=r1.1).contains(&r2.0) || (r1.0..=r1.1).contains(&r2.1)
            }
            contains(r1, r2) || contains(r2, r1)
        })
        .count()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
