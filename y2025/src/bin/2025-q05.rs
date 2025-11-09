use std::{
    env, fs,
    io::{self, Read},
    str::FromStr,
};

use itertools::{Itertools, MinMaxResult};

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

#[derive(Debug, PartialEq, Eq)]
enum Bone {
    Start(Int),
    Left(Int, Int),
    Right(Int, Int),
    Full(Int, Int, Int),
}

impl Bone {
    fn mut_next(&mut self, i: Int) -> bool {
        *self = match self {
            Bone::Start(o) if i < *o => Bone::Left(i, *o),
            Bone::Start(o) if *o < i => Bone::Right(*o, i),
            Bone::Left(ol, o) if *o < i => Bone::Full(*ol, *o, i),
            Bone::Right(o, or) if i < *o => Bone::Full(i, *o, *or),
            _ => return false,
        };
        true
    }

    fn spine(&self) -> Int {
        match self {
            Bone::Start(v) => *v,
            Bone::Left(_, v) => *v,
            Bone::Right(v, _) => *v,
            Bone::Full(_, v, _) => *v,
        }
    }

    fn level(&self) -> Int {
        match self {
            Bone::Start(v) => *v,
            Bone::Left(v1, v2) | Bone::Right(v1, v2) => v1 * (10 as Int).pow(v2.ilog10() + 1) + v2,
            Bone::Full(v1, v2, v3) => {
                v1 * (10 as Int).pow(v2.ilog2() + 1) + v2 * (10 as Int).pow(v3.ilog10() + 1) + v3
            }
        }
    }
}

impl PartialOrd for Bone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.level().cmp(&other.level())
    }
}

struct Sword(Int, Vec<Bone>);

impl Sword {
    fn quality(&self) -> Int {
        self.1.iter().map(|bone| bone.spine()).fold(0, |acc, bone| {
            acc * (10 as Int).pow(bone.ilog10() + 1) + bone
        })
    }

    fn checksum(&self, idx: usize) -> Int {
        (idx as Int + 1) * self.0
    }
}

impl std::fmt::Debug for Sword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.quality()))
    }
}

impl PartialEq for Sword {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for Sword {}

impl PartialOrd for Sword {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Sword {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.quality().cmp(&other.quality()) {
            std::cmp::Ordering::Equal => self.1.cmp(&other.1),
            cmp => cmp,
        }
    }
}

#[derive(Debug)]
enum SwordParseErr {
    InvalidNum,
    NoId,
    NoSpine,
}

impl FromStr for Sword {
    type Err = SwordParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SwordParseErr as E;
        let mut num_list = s
            .split([':', ','])
            .map(|n| n.parse::<Int>().map_err(|_| E::InvalidNum));

        let id = num_list.next().ok_or(E::NoId)??;
        let first = num_list.next().ok_or(E::NoSpine)??;
        let mut fishbone = vec![Bone::Start(first)];
        for n in num_list {
            let n = n?;
            // try each bone before inserting new.
            if let Some(()) = fishbone
                .iter_mut()
                .try_for_each(|b| (!b.mut_next(n)).then_some(()))
            {
                fishbone.push(Bone::Start(n));
            }
        }
        Ok(Sword(id, fishbone))
    }
}

fn parse(inp: &str) -> Vec<Sword> {
    inp.split('\n')
        .map(|line| line.parse::<Sword>().expect("Invalid sword"))
        .collect::<Vec<_>>()
}

fn main() -> io::Result<()> {
    let inp = read_input_as_str()?;
    let inp = inp.trim();
    let mut swords = parse(inp);
    let part1 = swords[0].quality();
    let part2 =
        if let MinMaxResult::MinMax(x, y) = swords.iter().map(|sword| sword.quality()).minmax() {
            y - x
        } else {
            0
        };
    swords.sort();
    let part3 = swords
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, s)| s.checksum(idx))
        .sum::<Int>();
    println!("Part1: {part1}, Part2: {part2}, Part3: {part3}");
    Ok(())
}
