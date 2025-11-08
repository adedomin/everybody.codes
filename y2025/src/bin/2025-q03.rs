use std::{
    env, fs,
    io::{self, Read},
};

use itertools::Itertools as _;

pub fn read_input_as_str() -> io::Result<String> {
    match env::args().nth(1) {
        Some(arg) => fs::read_to_string(arg),
        None => {
            let mut buf = String::new();
            io::stdin().lock().read_to_string(&mut buf)?;
            Ok(buf)
        }
    }
}

fn parse(inp: &str) -> Vec<i32> {
    inp.split(',')
        .map(|n| n.parse::<i32>().expect("number"))
        .collect::<Vec<_>>()
}

fn p1(mut inp: Vec<i32>, upto: usize) -> i32 {
    inp.sort();
    inp.into_iter().dedup().take(upto).sum()
}

fn p3(mut inp: Vec<i32>) -> usize {
    inp.sort();
    inp.into_iter()
        .dedup_with_count()
        .max_by_key(|(cnt, _)| *cnt)
        .expect("List cannot be empty.")
        .0
}

fn main() -> io::Result<()> {
    let inp = read_input_as_str()?;
    let inp = inp.trim();
    let inp = parse(inp);
    let part1 = p1(inp.clone(), inp.len());
    let part2 = p1(inp.clone(), 20);
    let part3 = p3(inp);
    println!("Part1: {part1}, Part2: {part2}, Part3: {part3}");
    Ok(())
}
