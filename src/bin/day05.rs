use std::str::FromStr;

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ").into_iter();

        parts.next();
        let count = parts
            .next()
            .ok_or(Self::Err::ParseError("failed to find count".to_string()))?;
        let count = count
            .parse::<usize>()
            .map_err(|_| Self::Err::ParseError(format!("failed to parse count {}", count)))?;

        parts.next();
        let from = parts
            .next()
            .ok_or(Self::Err::ParseError("failed to find from".to_string()))?;
        let from = from
            .parse::<usize>()
            .map_err(|_| Self::Err::ParseError(format!("failed to parse from {}", from)))?
            - 1;

        parts.next();
        let to = parts
            .next()
            .ok_or(Self::Err::ParseError("failed to find to".to_string()))?;
        let to = to
            .parse::<usize>()
            .map_err(|_| Self::Err::ParseError(format!("failed to parse to {}", to)))?
            - 1;

        return Ok(Move { count, from, to });
    }
}

impl Move {
    fn act9000(self: &Self, mut stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let from = &mut stacks[self.from];
        let mut moved = from.drain(0..self.count).collect::<Vec<char>>();

        moved.reverse();

        let to = &mut stacks[self.to];
        to.splice(0..0, moved);

        return stacks;
    }

    fn act9001(self: &Self, mut stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let from = &mut stacks[self.from];
        let moved = from.drain(0..self.count).collect::<Vec<char>>();

        let to = &mut stacks[self.to];
        to.splice(0..0, moved);

        return stacks;
    }
}

fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    if matrix[0].len() == 0 {
        return vec![];
    }

    let (h, t) = matrix
        .into_iter()
        .fold((Vec::new(), Vec::new()), |(mut hacc, mut tacc), line| {
            let (h, t) = line.split_at(1);
            hacc.extend_from_slice(h);
            tacc.push(t.to_vec());
            (hacc, tacc)
        });

    let mut t = transpose(t);
    t.insert(0, h);
    return t;
}

fn parse_stacks(input: impl AsRef<str>) -> Result<Vec<Vec<char>>, aoc::error::Error> {
    let lines = input
        .as_ref()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let lines = transpose(lines);

    let mut stacks = Vec::new();
    for i in (1..lines.len()).step_by(4) {
        let line = &lines[i];
        let line = &line[..line.len() - 1];
        stacks.push(
            line.to_vec()
                .into_iter()
                .filter(|c| c.is_alphabetic())
                .collect::<Vec<char>>(),
        );
    }

    return Ok(stacks);
}

fn parse_input(input: impl AsRef<str>) -> (Vec<Vec<char>>, Vec<Move>) {
    let (stacks, moves) = input
        .as_ref()
        .split_once("\n\n")
        .expect("to have correct input");

    return (
        parse_stacks(stacks).expect("to have correct stacks"),
        aoc::parsing::lines_to_vec::<Move>(moves).expect("to have correct moves"),
    );
}

fn part1(input: &(Vec<Vec<char>>, Vec<Move>)) -> String {
    let (stacks, moves) = input;
    let mut stacks = stacks.clone();

    for m in moves {
        stacks = m.act9000(stacks);
    }

    return stacks
        .into_iter()
        .flat_map(|s| s.first().copied())
        .collect::<String>();
}

fn part2(input: &(Vec<Vec<char>>, Vec<Move>)) -> String {
    let (stacks, moves) = input;
    let mut stacks = stacks.clone();

    for m in moves {
        stacks = m.act9001(stacks);
    }

    return stacks
        .into_iter()
        .flat_map(|s| s.first().copied())
        .collect::<String>();
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
        let input = super::parse_input("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2");

        assert_eq!(super::part1(&input), "CMZ");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2");

        assert_eq!(super::part2(&input), "MCD");
    }
}
