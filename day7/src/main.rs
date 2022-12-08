use std::{collections::BTreeMap, io::Read};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "cmd.pest"]
struct CmdParser;

#[derive(Default)]
struct FileSystem {
    root: BTreeMap<String, Entry>,
}

enum Entry {
    File(usize),
    Directory(BTreeMap<String, Entry>),
}

impl FileSystem {
    fn parse(commands: &str) -> Self {
        let mut fs = Self::default();
        let mut pwd = vec![];

        let f = CmdParser::parse(Rule::file, commands)
            .unwrap()
            .next()
            .unwrap();

        for pair in f.into_inner() {
            if pair.as_rule() == Rule::EOI {
                return fs;
            }
            let cmd = pair.into_inner().next().unwrap();
            match cmd.as_rule() {
                Rule::cd => {
                    let path = cmd.into_inner().next().unwrap().as_str();
                    match path {
                        "/" => {
                            pwd.clear();
                        }
                        ".." => {
                            pwd.pop();
                        }
                        _ => {
                            pwd.push(path);
                        }
                    }
                }
                Rule::ls => {
                    let mut children = cmd.into_inner().peekable();
                    while children.peek().is_some() {
                        let ty = children.next().unwrap();
                        let name = children.next().unwrap();
                        match ty.as_rule() {
                            Rule::dir => {}
                            Rule::number => {
                                let file_size = ty.as_str().parse::<usize>().unwrap();
                                fs.resolve_dir(pwd.as_slice())
                                    .insert(name.as_str().to_owned(), Entry::File(file_size));
                            }
                            _ => panic!(),
                        }
                    }
                }
                _ => panic!(),
            }
        }

        panic!()
    }

    fn resolve_dir<'a>(&'a mut self, path: &[&str]) -> &'a mut BTreeMap<String, Entry> {
        fn internal<'a>(
            head: &'a mut BTreeMap<String, Entry>,
            path: &[&str],
        ) -> &'a mut BTreeMap<String, Entry> {
            if let Some(segment) = path.first() {
                if !head.contains_key(*segment) {
                    head.insert((*segment).to_owned(), Entry::Directory(BTreeMap::new()));
                }
                match head.get_mut(*segment) {
                    Some(Entry::Directory(dir)) => internal(dir, &path[1..]),
                    _ => panic!(),
                }
            } else {
                head
            }
        }

        internal(&mut self.root, path)
    }

    fn iter_dirs(&self) -> impl Iterator<Item = &BTreeMap<String, Entry>> + '_ {
        fn internal(
            curr: &BTreeMap<String, Entry>,
        ) -> Box<dyn Iterator<Item = &BTreeMap<String, Entry>> + '_> {
            Box::new(
                std::iter::once(curr).chain(curr.values().flat_map(|entry| match entry {
                    Entry::File(_) => Box::new(std::iter::empty()),
                    Entry::Directory(dir) => internal(dir),
                })),
            )
        }
        internal(&self.root)
    }
}

fn size_of_dir(dir: &BTreeMap<String, Entry>) -> usize {
    let mut total = 0;
    for entry in dir.values() {
        match entry {
            Entry::File(size) => total += size,
            Entry::Directory(child) => total += size_of_dir(child),
        }
    }
    total
}

fn part1() -> usize {
    let mut commands = String::new();
    std::io::BufReader::new(std::fs::File::open("day7/input.txt").unwrap())
        .read_to_string(&mut commands)
        .unwrap();

    let fs = FileSystem::parse(commands.as_str());

    fs.iter_dirs()
        .map(size_of_dir)
        .filter(|size| *size <= 100000)
        .sum()
}

fn part2() -> usize {
    let mut commands = String::new();
    std::io::BufReader::new(std::fs::File::open("day7/input.txt").unwrap())
        .read_to_string(&mut commands)
        .unwrap();

    let fs = FileSystem::parse(commands.as_str());
    let total_size = 70000000;
    let used_size = size_of_dir(&fs.root);
    let required_free = 30000000;
    let used_max = total_size - required_free;
    let required_to_remove = used_size - used_max;

    fs.iter_dirs()
        .map(size_of_dir)
        .filter(|size| *size >= required_to_remove)
        .min()
        .unwrap()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
