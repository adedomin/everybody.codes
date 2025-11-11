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
const SWORD_KNIGHT_IDX: usize = 0;
const LIM: usize = 1000;
const REPEAT: usize = 1000;

fn p3_range(idx: usize, len: usize) -> std::ops::Range<usize> {
    let start = idx.saturating_sub(LIM);
    let end = (idx + LIM + 1).min(len);
    start..end
}

fn parse(list: &[u8], p3: bool) -> [usize; ALPHABET_SZ] {
    let mut hs = [0; ALPHABET_SZ];
    let len = list.len();
    list.iter()
        .enumerate()
        .flat_map(|(idx, knight)| knight.is_ascii_uppercase().then_some((idx, *knight)))
        .for_each(|(idx, knight)| {
            let entry = &mut hs[(knight - ALPHA_START) as usize];
            let range = if p3 { p3_range(idx, len) } else { idx..len };
            *entry += list[range]
                .iter()
                .filter(|&&k| k.is_ascii_lowercase() && k.to_ascii_uppercase() == knight)
                .count();
        });
    hs
}

fn main() -> io::Result<()> {
    let inp = read_input_as_str()?;
    let inp = inp.trim();
    let listof = parse(inp.as_bytes(), false);
    let part1 = listof[SWORD_KNIGHT_IDX];
    let part2 = listof.iter().sum::<usize>();

    let inp = inp.repeat(REPEAT);
    let part3 = parse(inp.as_bytes(), true).iter().sum::<usize>();
    println!("Part1: {part1}, Part2: {part2}, Part3: {part3}");
    Ok(())
}
