use std::{io::BufRead, ops::Add};

fn part1() -> usize {
    std::io::BufReader::new(std::fs::File::open("day2/input.txt").unwrap())
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap()
        .into_iter()
        .map(|s| {
            let (them, you) = s.split_at(1);
            (match you {
                " X" => 1,
                " Y" => 2,
                " Z" => 3,
                _ => panic!(),
            }) + (match (them, you) {
                ("A", " Y") | ("B", " Z") | ("C", " X") => 6,
                ("B", " X") | ("C", " Y") | ("A", " Z") => 0,
                _ => 3,
            })
        })
        .sum()
}

fn part2() -> usize {
    #[derive(Clone, Copy)]
    enum Outcome {
        Win,
        Lose,
        Draw,
    }
    impl Outcome {
        fn value(self) -> usize {
            match self {
                Self::Win => 6,
                Self::Lose => 0,
                Self::Draw => 3,
            }
        }
    }
    #[derive(Clone, Copy)]
    enum Choice {
        Rock,
        Paper,
        Scissors,
    }
    impl Choice {
        fn value(self) -> usize {
            match self {
                Self::Rock => 1,
                Self::Paper => 2,
                Self::Scissors => 3,
            }
        }
    }
    impl Add<Outcome> for Choice {
        type Output = Choice;

        fn add(self, rhs: Outcome) -> Self::Output {
            match (self, rhs) {
                (_, Outcome::Draw) => self,
                (Self::Paper, Outcome::Win) => Self::Scissors,
                (Self::Rock, Outcome::Win) => Self::Paper,
                (Self::Scissors, Outcome::Win) => Self::Rock,
                (Self::Paper, Outcome::Lose) => Self::Rock,
                (Self::Rock, Outcome::Lose) => Self::Scissors,
                (Self::Scissors, Outcome::Lose) => Self::Paper,
            }
        }
    }

    std::io::BufReader::new(std::fs::File::open("day2/input.txt").unwrap())
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap()
        .into_iter()
        .map(|s| {
            let (them, you) = s.split_at(1);
            let their_choice = match them {
                "A" => Choice::Rock,
                "B" => Choice::Paper,
                "C" => Choice::Scissors,
                _ => panic!(),
            };
            let outcome = match you {
                " X" => Outcome::Lose,
                " Y" => Outcome::Draw,
                " Z" => Outcome::Win,
                _ => panic!(),
            };
            (their_choice + outcome).value() + outcome.value()
        })
        .sum()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
