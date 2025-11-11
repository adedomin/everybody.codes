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

const ALPHA_START: u8 = b'A';
const ALPHABET_SZ: usize = 26;

fn parse(list: &[u8]) -> [usize; ALPHABET_SZ] {
    let mut hs = [0; ALPHABET_SZ];
    list.iter()
        .enumerate()
        .flat_map(|(idx, knight)| knight.is_ascii_uppercase().then_some((idx, *knight)))
        .for_each(|(idx, knight)| {
            let entry = &mut hs[(knight - ALPHA_START) as usize];
            *entry += list[idx..]
                .iter()
                .filter(|&&k| k.is_ascii_lowercase() && k.to_ascii_uppercase() == knight)
                .count();
        });
    hs
}

const LIM: usize = 1000;
const REPEAT: usize = 1000;

fn p3_range(idx: usize, len: usize) -> std::ops::Range<usize> {
    let start = idx.saturating_sub(LIM);
    let end = (idx + LIM + 1).min(len);
    start..end
}

fn p3(list: &[u8]) -> usize {
    let mut hs = [0; ALPHABET_SZ];
    list.iter()
        .enumerate()
        .flat_map(|(idx, novice)| novice.is_ascii_lowercase().then_some((idx, *novice)))
        .for_each(|(idx, novice)| {
            let entry = &mut hs[(novice - b'a') as usize];
            *entry += list[p3_range(idx, list.len())]
                .iter()
                .filter(|&&k| k.is_ascii_uppercase() && k.to_ascii_lowercase() == novice)
                .count();
        });
    hs.iter().sum::<usize>()
}

const SWORD_KNIGHT_IDX: usize = 0;

fn main() -> io::Result<()> {
    let inp = read_input_as_str()?;
    let inp = inp.trim();
    let listof = parse(inp.as_bytes());
    let part1 = listof[SWORD_KNIGHT_IDX];
    let part2 = listof.iter().sum::<usize>();

    let inp = inp.repeat(REPEAT);
    let part3 = p3(inp.as_bytes());
    println!("Part1: {part1}, Part2: {part2}, Part3: {part3}");
    Ok(())
}
