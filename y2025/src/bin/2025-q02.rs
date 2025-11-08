use std::{
    env, fs,
    io::{self, Read},
};

use num::Complex;
use regex::Regex;

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

type Int = i64;

fn parse(inp: &str) -> Complex<Int> {
    let re = Regex::new(r"^A=\[*(?<real>-?[[:digit:]]+),(?<imag>-?[[:digit:]]+)\]$").unwrap();
    let caps = re
        .captures(inp)
        .expect("invalid input, must be in the shape of <IDENT>=[<REAL>,<IMGAGINARY>]");
    let re = caps.name("real").unwrap().as_str().parse::<Int>().unwrap();
    let im = caps.name("imag").unwrap().as_str().parse::<Int>().unwrap();
    Complex { re, im }
}

const P1_DIV: Int = 10;
const LIM: std::ops::Range<Int> = -1_000_000..1_000_001;

fn p1<const DIV: Int>(i: Complex<Int>, cycles: usize) -> Option<Complex<Int>> {
    std::iter::successors(Some(Complex::new(0, 0)), |num| {
        let num = num * num / DIV + i;
        (LIM.contains(&num.re) && LIM.contains(&num.im)).then_some(num)
    })
    .nth(cycles)
}

const P2_DIV: Int = 100_000;
const P2_OFF: Complex<Int> = Complex::new(1_000, 1_000);
const P2_STEP: usize = 10;
const P3_STEP: usize = 1;

fn p2(start: Complex<Int>, step: usize) -> usize {
    let end = start + P2_OFF;
    (start.im..end.im + 1)
        .step_by(step)
        .flat_map(|y| {
            (start.re..end.re + 1)
                .step_by(step)
                .map(move |x| Complex::new(x, y))
        })
        .flat_map(|xy| p1::<P2_DIV>(xy, 100))
        .count()
}

fn main() -> io::Result<()> {
    let inp = read_input_as_str()?;
    let inp = inp.trim();
    let inp = parse(inp);
    let Complex { re, im } = p1::<P1_DIV>(inp, 3).unwrap_or(Complex::new(0, 0));
    let part2 = p2(inp, P2_STEP);
    let part3 = p2(inp, P3_STEP);
    println!("Part1 [{re},{im}], Part2: {part2}, Part3:: {part3}");
    Ok(())
}
