use std::io::Read;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Item {
    List(Vec<Item>),
    Number(u32),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn cmp_slice(l1: &[impl AsRef<Item>], l2: &[impl AsRef<Item>]) -> std::cmp::Ordering {
            for (i1, i2) in l1.iter().zip(l2.iter()) {
                let res = i1.as_ref().cmp(i2.as_ref());
                if res != std::cmp::Ordering::Equal {
                    return res;
                }
            }
            l1.len().cmp(&l2.len())
        }

        match (self, other) {
            (Item::List(l1), Item::List(l2)) => cmp_slice(l1.as_slice(), l2.as_slice()),
            (Item::List(l1), Item::Number(_)) => cmp_slice(l1.as_slice(), &[other]),
            (Item::Number(_), Item::List(l2)) => cmp_slice(&[self], l2.as_slice()),
            (Item::Number(n1), Item::Number(n2)) => n1.cmp(n2),
        }
    }
}

impl AsRef<Item> for Item {
    fn as_ref(&self) -> &Item {
        self
    }
}

#[derive(Parser)]
#[grammar = "lists.pest"]
struct ListsParser;

fn parse_items(text: &str) -> Vec<Item> {
    fn parse_item(item: Pair<Rule>) -> Item {
        match item.as_rule() {
            Rule::number => Item::Number(item.as_str().parse().unwrap()),
            Rule::list => {
                let mut ls = Vec::new();
                for child in item.into_inner() {
                    ls.push(parse_item(child.into_inner().next().unwrap()));
                }
                Item::List(ls)
            }
            _ => panic!(),
        }
    }

    let mut res = Vec::new();

    let f = match ListsParser::parse(Rule::file, text) {
        Ok(mut f) => f.next().unwrap(),
        Err(e) => panic!("{e}"),
    };

    for item in f.into_inner() {
        if item.as_rule() == Rule::EOI {
            return res;
        }

        res.push(parse_item(item.into_inner().next().unwrap()));
    }

    panic!()
}

fn part1() -> usize {
    let mut text = String::new();
    std::io::BufReader::new(std::fs::File::open("day13/input.txt").unwrap())
        .read_to_string(&mut text)
        .unwrap();

    let items = parse_items(text.as_str());

    let mut res = 0;
    for (pairs, idx) in items.chunks_exact(2).zip(1..) {
        match pairs[0].cmp(&pairs[1]) {
            std::cmp::Ordering::Less => {
                res += idx;
            }
            std::cmp::Ordering::Equal => panic!(),
            std::cmp::Ordering::Greater => {}
        }
    }

    res
}

fn part2() -> usize {
    let mut text = String::new();
    std::io::BufReader::new(std::fs::File::open("day13/input.txt").unwrap())
        .read_to_string(&mut text)
        .unwrap();

    let mut items = parse_items(text.as_str());

    let d1 = Item::List(vec![Item::List(vec![Item::Number(2)])]);
    let d2 = Item::List(vec![Item::List(vec![Item::Number(6)])]);

    items.push(d1.clone());
    items.push(d2.clone());

    items.sort();

    let c1 = items
        .iter()
        .enumerate()
        .find(|(_, item)| *item == &d1)
        .unwrap()
        .0
        + 1;
    let c2 = items
        .iter()
        .enumerate()
        .find(|(_, item)| *item == &d2)
        .unwrap()
        .0
        + 1;

    c1 * c2
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
