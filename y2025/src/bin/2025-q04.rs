use std::{
    env, fs,
    io::{self, Read},
    str::FromStr,
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

type Num = f64;

enum Gear {
    Single(Num),
    Double(Num, Num),
}

impl FromStr for Gear {
    type Err = <Num as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((out, inner)) = s.split_once('|') {
            Ok(Gear::Double(out.parse::<Num>()?, inner.parse::<Num>()?))
        } else {
            Ok(Gear::Single(s.parse::<Num>()?))
        }
    }
}

fn parse(inp: &str) -> Vec<Gear> {
    inp.split('\n')
        .map(|num| num.parse().expect("Valid gear"))
        .collect::<Vec<_>>()
}

fn get_ratio(gears: &[Gear]) -> Num {
    gears
        .iter()
        .tuple_windows()
        .map(|(x, y)| match (x, y) {
            (Gear::Single(x), Gear::Single(y)) => x / y,
            (Gear::Single(x), Gear::Double(y, _)) => x / y,
            (Gear::Double(_, x), Gear::Single(y)) => x / y,
            (Gear::Double(_, x), Gear::Double(y, _)) => x / y,
        })
        .product::<Num>()
}

const P1_TARGET: Num = 2025f64;
const P2_TARGET: Num = 10_000_000_000_000f64;
const P3_TARGET: Num = 100f64;

fn main() -> io::Result<()> {
    let inp = read_input_as_str()?;
    let inp = inp.trim();
    let gears = parse(inp);
    let ratio_last = get_ratio(&gears);
    // wtf?
    let part1 = (ratio_last * P1_TARGET).floor();
    // lol?
    let part2 = (P2_TARGET / ratio_last).ceil();
    let part3 = (ratio_last * P3_TARGET).floor();
    println!("Part1: {part1}, Part2: {part2}, Part3: {part3}");
    Ok(())
}
