use std::{iter::Peekable, str::FromStr};

#[derive(Debug, Clone)]
enum PacketData {
    List(Vec<PacketData>),
    Elem(usize),
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        match self {
            PacketData::Elem(value) => match other {
                PacketData::Elem(other_value) => value.eq(other_value),
                PacketData::List(other_vec) => PacketData::List(vec![PacketData::Elem(*value)])
                    .eq(&PacketData::List(other_vec.to_vec())),
            },
            PacketData::List(vec) => match other {
                PacketData::Elem(other_value) => PacketData::List(vec.to_vec())
                    .eq(&PacketData::List(vec![PacketData::Elem(*other_value)])),
                PacketData::List(other_vec) => {
                    for (x, y) in vec.into_iter().zip(other_vec.into_iter()) {
                        if !x.eq(y) {
                            return false;
                        }
                    }

                    return vec.len().eq(&other_vec.len());
                }
            },
        }
    }
}

impl Eq for PacketData {}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            PacketData::Elem(value) => match other {
                PacketData::Elem(other_value) => value.cmp(other_value),
                PacketData::List(other_vec) => PacketData::List(vec![PacketData::Elem(*value)])
                    .cmp(&PacketData::List(other_vec.to_vec())),
            },
            PacketData::List(vec) => match other {
                PacketData::Elem(other_value) => PacketData::List(vec.to_vec())
                    .cmp(&PacketData::List(vec![PacketData::Elem(*other_value)])),
                PacketData::List(other_vec) => {
                    for (x, y) in vec.into_iter().zip(other_vec.into_iter()) {
                        if !x.eq(y) {
                            return x.cmp(y);
                        }
                    }

                    return vec.len().cmp(&other_vec.len());
                }
            },
        }
    }
}

impl FromStr for PacketData {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return from_str_helper(&mut s.chars().into_iter().peekable());
    }
}

fn from_str_helper<I>(chars: &mut Peekable<I>) -> Result<PacketData, aoc::error::Error>
where
    I: Iterator<Item = char>,
{
    match chars.next() {
        Some('[') => {
            if let Some(']') = chars.peek() {
                chars.next();
                return Ok(PacketData::List(vec![]));
            }

            let mut data = Vec::new();

            loop {
                data.push(from_str_helper(chars)?);

                if let Some(']') = chars.next() {
                    break;
                }
            }

            return Ok(PacketData::List(data));
        }
        Some(c) => {
            let mut digits = vec![c];

            loop {
                if chars.peek() == Some(&&']') || chars.peek() == Some(&&',') {
                    break;
                }

                if let Some(c) = chars.next() {
                    digits.push(c);
                }
            }

            let value = digits
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .map_err(|e| aoc::error::Error::ParseError(e.to_string()))?;

            return Ok(PacketData::Elem(value));
        }
        None => unreachable!(),
    }
}

#[derive(Debug)]
struct Packet {
    lhs: PacketData,
    rhs: PacketData,
}

impl FromStr for Packet {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().into_iter();

        let lhs = lines
            .next()
            .ok_or(Self::Err::ParseError("expect first line".to_string()))?
            .parse::<PacketData>()?;
        let rhs = lines
            .next()
            .ok_or(Self::Err::ParseError("expect second line".to_string()))?
            .parse::<PacketData>()?;

        return Ok(Packet { lhs, rhs });
    }
}

fn parse_input(input: impl AsRef<str>) -> Vec<Packet> {
    input
        .as_ref()
        .split("\n\n")
        .map(|p| p.parse::<Packet>())
        .collect::<Result<Vec<Packet>, _>>()
        .expect("aoc to have correct input")
}

fn part1(input: &Vec<Packet>) -> String {
    (1..input.len() + 1)
        .filter(|&i| input[i - 1].lhs < input[i - 1].rhs)
        .sum::<usize>()
        .to_string()
}

fn part2(input: &Vec<Packet>) -> String {
    let mut packets = input
        .into_iter()
        .flat_map(|p| vec![p.lhs.clone(), p.rhs.clone()])
        .collect::<Vec<PacketData>>();

    packets.push(PacketData::List(vec![PacketData::Elem(2)]));
    packets.push(PacketData::List(vec![PacketData::Elem(6)]));

    packets.sort();

    let mut result = 1;

    for i in 0..packets.len() {
        if packets[i] == PacketData::List(vec![PacketData::Elem(2)]) {
            result *= i + 1;
        }
        if packets[i] == PacketData::List(vec![PacketData::Elem(6)]) {
            result *= i + 1;
        }
    }

    return result.to_string();
}

fn main() {
    let input = parse_input(aoc::input::read_from_stdin());

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example1() {
        let input = super::parse_input(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        );

        assert_eq!(super::part1(&input), "13");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        );

        assert_eq!(super::part2(&input), "140");
    }
}
