use std::{
    env, fs,
    io::{self, Read},
};

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

fn parse<'a>(inp: &'a str) -> (Vec<&'a str>, Vec<isize>) {
    let (names, moves) = inp
        .split_once("\n\n")
        .expect("double newline delimited list");
    let names = names.split(',').collect::<Vec<&'a str>>();
    assert!(!names.is_empty(), "Names can't be empty");
    let moves = moves
        .split(',')
        .map(|s| {
            if let Some(num) = s.strip_prefix('R') {
                num.parse::<isize>().expect("Number")
            } else if let Some(num) = s.strip_prefix('L') {
                -num.parse::<isize>().expect("Number")
            } else {
                panic!("Not a Movement!");
            }
        })
        .collect::<Vec<isize>>();
    (names, moves)
}

fn p1<'a>(names: &[&'a str], moves: &[isize]) -> &'a str {
    let idx = moves.iter().fold(0isize, |acc, idx| {
        (acc + idx).clamp(0, names.len() as isize - 1)
    });
    names[idx as usize]
}

fn p2<'a>(names: &[&'a str], moves: &[isize]) -> &'a str {
    let bounds = names.len() as isize;
    let idx = moves
        .iter()
        .fold(0isize, |acc, idx| (acc + idx).rem_euclid(bounds));
    names[idx as usize]
}

fn p3<'a>(mut names: Vec<&'a str>, moves: &[isize]) -> &'a str {
    let bounds = names.len() as isize;
    moves
        .iter()
        .for_each(|&idx| names.swap(0, idx.rem_euclid(bounds) as usize));
    names.swap_remove(0)
}

fn main() -> io::Result<()> {
    let inp = read_input_as_str()?;
    let inp = inp.trim();
    let (names, pos) = parse(inp);
    let p1 = p1(&names, &pos);
    let p2 = p2(&names, &pos);
    let p3 = p3(names, &pos);
    println!("Part1: {p1}, Part2: {p2}, Part3: {p3}");
    Ok(())
}
