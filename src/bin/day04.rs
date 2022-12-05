use std::str::FromStr;

#[derive(Debug)]
struct Range {
    low: usize,
    high: usize,
}

impl FromStr for Range {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (low, high) = s
            .split_once("-")
            .ok_or_else(|| Self::Err::ParseError(s.to_owned()))?;

        return Ok(Range {
            low: low
                .parse()
                .map_err(|_| Self::Err::ParseError(s.to_owned()))?,
            high: high
                .parse()
                .map_err(|_| Self::Err::ParseError(s.to_owned()))?,
        });
    }
}

#[derive(Debug)]
struct Pair {
    r1: Range,
    r2: Range,
}

impl FromStr for Pair {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (r1, r2) = s
            .split_once(",")
            .ok_or_else(|| Self::Err::ParseError(s.to_owned()))?;

        return Ok(Pair {
            r1: r1.parse()?,
            r2: r2.parse()?,
        });
    }
}

fn parse_input(input: impl AsRef<str>) -> Vec<Pair> {
    aoc::parsing::lines_to_vec(input).unwrap()
}

fn part1(pairs: &Vec<Pair>) -> String {
    pairs
        .into_iter()
        .filter(|pair| {
            (pair.r1.low <= pair.r2.low && pair.r2.high <= pair.r1.high)
                || (pair.r2.low <= pair.r1.low && pair.r1.high <= pair.r2.high)
        })
        .count()
        .to_string()
}

fn part2(pairs: &Vec<Pair>) -> String {
    pairs
        .into_iter()
        .filter(|pair| pair.r1.low <= pair.r2.high && pair.r2.low <= pair.r1.high)
        .count()
        .to_string()
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
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        );

        assert_eq!(super::part1(&input), "2");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        );

        assert_eq!(super::part2(&input), "4");
    }
}
