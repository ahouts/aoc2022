use pest::Parser;
use pest_derive::Parser;
use std::io::Read;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Ins {
    NoOp,
    Addx(isize),
}

#[derive(Parser)]
#[grammar = "prog.pest"]
struct ProgramParser;

fn parse_program(prog: &str) -> Vec<Ins> {
    let mut res = Vec::new();

    let f = ProgramParser::parse(Rule::file, prog)
        .unwrap()
        .next()
        .unwrap();

    for pair in f.into_inner() {
        if pair.as_rule() == Rule::EOI {
            return res;
        }
        let ins = pair.into_inner().next().unwrap();
        match ins.as_rule() {
            Rule::noop => res.push(Ins::NoOp),
            Rule::addx => res.push(Ins::Addx(
                ins.into_inner().next().unwrap().as_str().parse().unwrap(),
            )),
            _ => panic!(),
        }
    }

    panic!()
}

fn part1() -> isize {
    let mut prog = String::new();
    std::io::BufReader::new(std::fs::File::open("day10/input.txt").unwrap())
        .read_to_string(&mut prog)
        .unwrap();

    let mut ins = parse_program(prog.as_str()).into_iter();

    let mut result = 0;
    let mut cycle = 1;
    let mut reg = 1;

    fn run_cycle(cycle: &mut isize, result: &mut isize, reg: isize) {
        if *cycle % 40 == 20 {
            *result += *cycle * reg;
        }
        *cycle += 1;
    }

    while cycle <= 220 {
        match ins.next().unwrap() {
            Ins::NoOp => {
                run_cycle(&mut cycle, &mut result, reg);
            }
            Ins::Addx(value) => {
                run_cycle(&mut cycle, &mut result, reg);
                run_cycle(&mut cycle, &mut result, reg);
                reg += value;
            }
        }
    }

    result
}

fn part2() -> String {
    let mut prog = String::new();
    std::io::BufReader::new(std::fs::File::open("day10/input.txt").unwrap())
        .read_to_string(&mut prog)
        .unwrap();

    let mut ins = parse_program(prog.as_str()).into_iter();

    let mut vbuffer = String::new();
    let mut cycle = 0;
    let mut reg = 1;

    fn run_cycle(vbuffer: &mut String, cycle: &mut isize, reg: isize) {
        if *cycle >= 240 {
            return;
        }
        if ((reg - 1)..=(reg + 1)).contains(&(*cycle % 40)) {
            vbuffer.push('■');
        } else {
            vbuffer.push(' ');
        }

        *cycle += 1;

        if *cycle % 40 == 0 {
            vbuffer.push('\n');
        }
    }

    while cycle < 240 {
        match ins.next().unwrap() {
            Ins::NoOp => {
                run_cycle(&mut vbuffer, &mut cycle, reg);
            }
            Ins::Addx(value) => {
                run_cycle(&mut vbuffer, &mut cycle, reg);
                run_cycle(&mut vbuffer, &mut cycle, reg);
                reg += value;
            }
        }
    }

    vbuffer
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
