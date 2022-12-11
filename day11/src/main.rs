use std::{collections::VecDeque, io::Read};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "monkey.pest"]
struct MonkeyParser;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: (Target, Op, Target),
    test_mod: usize,
    true_target: usize,
    false_target: usize,
    inspection_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Number(usize),
    Old,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn parse_monkeys(text: &str) -> Vec<Monkey> {
    let mut res = Vec::new();

    let f = match MonkeyParser::parse(Rule::file, text) {
        Ok(mut f) => f.next().unwrap(),
        Err(e) => panic!("{e}"),
    };

    for monkey in f.into_inner() {
        if monkey.as_rule() == Rule::EOI {
            return res;
        }

        let mut parts = monkey.into_inner();
        parts.next();

        let mut starting_items = VecDeque::<usize>::new();
        for item in parts.next().unwrap().into_inner() {
            starting_items.push_back(item.as_str().parse().unwrap());
        }

        fn parse_target(pair: Pair<Rule>) -> Target {
            if pair.as_rule() == Rule::old {
                Target::Old
            } else {
                Target::Number(pair.as_str().parse().unwrap())
            }
        }

        let mut operation = parts.next().unwrap().into_inner();
        let t1 = parse_target(operation.next().unwrap());
        let op = match operation.next().unwrap().as_str() {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => panic!(),
        };
        let t2 = parse_target(operation.next().unwrap());

        let mut test = parts.next().unwrap().into_inner();
        let test_mod = test.next().unwrap().as_str().parse().unwrap();
        let true_target = test.next().unwrap().as_str().parse().unwrap();
        let false_target = test.next().unwrap().as_str().parse().unwrap();

        res.push(Monkey {
            items: starting_items,
            operation: (t1, op, t2),
            test_mod,
            true_target,
            false_target,
            inspection_count: 0,
        });
    }

    panic!()
}

fn part1() -> usize {
    let mut text = String::new();
    std::io::BufReader::new(std::fs::File::open("day11/input.txt").unwrap())
        .read_to_string(&mut text)
        .unwrap();

    let mut monkeys = parse_monkeys(text.as_str());

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(old) = monkeys[i].items.pop_front() {
                monkeys[i].inspection_count += 1;
                let lhs = match monkeys[i].operation.0 {
                    Target::Number(n) => n,
                    Target::Old => old,
                };
                let rhs = match monkeys[i].operation.2 {
                    Target::Number(n) => n,
                    Target::Old => old,
                };
                let new = match monkeys[i].operation.1 {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                };
                let bored = new / 3;
                let target = if bored % monkeys[i].test_mod == 0 {
                    monkeys[i].true_target
                } else {
                    monkeys[i].false_target
                };
                monkeys[target].items.push_back(bored);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspection_count);
    let n_monkeys = monkeys.len();

    monkeys[n_monkeys - 1].inspection_count * monkeys[n_monkeys - 2].inspection_count
}

fn part2() -> usize {
    let mut text = String::new();
    std::io::BufReader::new(std::fs::File::open("day11/input.txt").unwrap())
        .read_to_string(&mut text)
        .unwrap();

    let mut monkeys = parse_monkeys(text.as_str());

    let test_mods = monkeys.iter().map(|m| m.test_mod).collect::<Vec<usize>>();

    fn lcm(numbers: &[usize]) -> usize {
        let mut lcm = 1;
        for &number in numbers {
            lcm = lcm * number / gcd(lcm, number);
        }
        lcm
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    let test_mod_lcm = lcm(test_mods.as_slice());

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some(old) = monkeys[i].items.pop_front() {
                monkeys[i].inspection_count += 1;
                let lhs = match monkeys[i].operation.0 {
                    Target::Number(n) => n,
                    Target::Old => old,
                };
                let rhs = match monkeys[i].operation.2 {
                    Target::Number(n) => n,
                    Target::Old => old,
                };
                let new = match monkeys[i].operation.1 {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                } % test_mod_lcm;
                let target = if new % monkeys[i].test_mod == 0 {
                    monkeys[i].true_target
                } else {
                    monkeys[i].false_target
                };
                monkeys[target].items.push_back(new);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspection_count);
    let n_monkeys = monkeys.len();

    monkeys[n_monkeys - 1].inspection_count * monkeys[n_monkeys - 2].inspection_count
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
