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

const KNIGHT_START: u8 = b'A';
const NOVICE_START: u8 = b'a';
const SWORD_KNIGHT_IDX: usize = 0;
const LIM: isize = 1000;
const REPEAT: usize = 1000;

fn is_knight(b: &u8) -> Option<usize> {
    (b'A'..=b'C')
        .contains(b)
        .then_some((b - KNIGHT_START) as usize)
}

fn is_novice(b: &u8) -> Option<usize> {
    (b'a'..=b'c')
        .contains(b)
        .then_some((b - NOVICE_START) as usize)
}

fn p1(list: &[u8]) -> [usize; 3] {
    let [mut pairs, mut histo] = [[0; 3]; 2];
    list.iter().for_each(|kn| {
        if let Some(i) = is_knight(kn) {
            histo[i] += 1;
        } else if let Some(i) = is_novice(kn) {
            pairs[i] += histo[i];
        }
    });
    pairs
}

fn p3(list: &[u8]) -> usize {
    let len = list.len() as isize;
    // since the problem calls for LIM 1000 and REPEAT 1000
    // it should never trigger for inputs, only examples.
    debug_assert!(
        LIM == REPEAT as isize || LIM <= (len * REPEAT as isize),
        "Cannot solve with current algo; input list is too short."
    );
    list.iter()
        .enumerate()
        .flat_map(|(i, k)| is_knight(k).map(|_| (i as isize, k)))
        .flat_map(|(i, k)| {
            let start = i - LIM;
            let end = i + LIM + 1;
            (start..end).map(move |i2| {
                if list[i2.rem_euclid(len) as usize] == k.to_ascii_lowercase() {
                    let off = if i2 < 0 { len - 1 - i2 } else { i2 };
                    // this knight will see this novice this many times.
                    // with the input repeated `REPEAT` times.
                    REPEAT - (off / len) as usize
                } else {
                    0
                }
            })
        })
        .sum()
}

fn main() -> io::Result<()> {
    let inp = read_input_as_str()?;
    let inp = inp.trim();
    let listof = p1(inp.as_bytes());
    let part1 = listof[SWORD_KNIGHT_IDX];
    let part2 = listof.iter().sum::<usize>();
    let part3 = p3(inp.as_bytes());
    println!("Part1: {part1}, Part2: {part2}, Part3: {part3}");
    Ok(())
}
