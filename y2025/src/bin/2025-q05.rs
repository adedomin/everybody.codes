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
    fn next(&self, i: Int) -> Option<Bone> {
        match self {
            Bone::Start(o) if i < *o => Some(Bone::Left(i, *o)),
            Bone::Start(o) if *o < i => Some(Bone::Right(*o, i)),
            Bone::Left(ol, o) if *o < i => Some(Bone::Full(*ol, *o, i)),
            Bone::Right(o, or) if i < *o => Some(Bone::Full(i, *o, *or)),
            _ => None,
        }
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

impl FromStr for Sword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, list) = s.split_once(':').ok_or(())?;
        let id = id.parse::<Int>().map_err(|_| ())?;
        let mut list_itr = list.split(',');
        let first = list_itr.next().ok_or(())?.parse::<Int>().map_err(|_| ())?;
        let mut fishbone = vec![Bone::Start(first)];
        for n in list_itr {
            let n = n.parse::<Int>().map_err(|_| ())?;
            if let Ok(()) = fishbone.iter_mut().try_for_each(|b| {
                if let Some(nb) = b.next(n) {
                    *b = nb;
                    Err(())
                } else {
                    Ok(())
                }
            }) {
                fishbone.push(Bone::Start(n));
            }
        }
        Ok(Sword(id, fishbone))
    }
}

fn parse(inp: &str) -> Vec<Sword> {
    inp.split('\n')
        .map(|line| line.parse::<Sword>().expect("Valid sword."))
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
