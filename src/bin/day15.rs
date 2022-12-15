use itertools::Itertools;
use std::str::FromStr;

const Y_LEVEL_1: i64 = 2000000;
const Y_LEVEL_2: i64 = 4000000;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        return Point { x, y };
    }

    fn manhattan(self: &Self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct Line {
    s: Point,
    b: Point,

    dist: i64,
}

impl Line {
    fn new(s: Point, b: Point) -> Self {
        Self {
            s,
            b,
            dist: s.manhattan(&b),
        }
    }
}

impl FromStr for Line {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(" ").into_iter();

        tokens.next();
        tokens.next();

        let sx = tokens
            .next()
            .ok_or(Self::Err::ParseError("expect sensor x token".to_string()))?
            .split_once("=")
            .ok_or(Self::Err::ParseError(
                "except sensor x token to have = sign".to_string(),
            ))?
            .1
            .split_once(",")
            .ok_or(Self::Err::ParseError(
                "except sensor x to be followed by , character".to_string(),
            ))?
            .0
            .parse::<i64>()
            .map_err(|e| Self::Err::ParseError(e.to_string()))?;

        let sy = tokens
            .next()
            .ok_or(Self::Err::ParseError("expect sensor y token".to_string()))?
            .split_once("=")
            .ok_or(Self::Err::ParseError(
                "except sensor y token to have = sign".to_string(),
            ))?
            .1
            .split_once(":")
            .ok_or(Self::Err::ParseError(
                "except sensor y to be followed by : character".to_string(),
            ))?
            .0
            .parse::<i64>()
            .map_err(|e| Self::Err::ParseError(e.to_string()))?;

        tokens.next();
        tokens.next();
        tokens.next();
        tokens.next();

        let bx = tokens
            .next()
            .ok_or(Self::Err::ParseError("expect beacon x token".to_string()))?
            .split_once("=")
            .ok_or(Self::Err::ParseError(
                "except beacon x token to have = sign".to_string(),
            ))?
            .1
            .split_once(",")
            .ok_or(Self::Err::ParseError(
                "except beacon x to be followed by , character".to_string(),
            ))?
            .0
            .parse::<i64>()
            .map_err(|e| Self::Err::ParseError(e.to_string()))?;

        let by = tokens
            .next()
            .ok_or(Self::Err::ParseError("expect beacon y token".to_string()))?
            .split_once("=")
            .ok_or(Self::Err::ParseError(
                "except beacon y token to have = sign".to_string(),
            ))?
            .1
            .parse::<i64>()
            .map_err(|e| Self::Err::ParseError(e.to_string()))?;

        return Ok(Line::new(Point::new(sx, sy), Point::new(bx, by)));
    }
}

fn parse_input(input: impl AsRef<str>) -> Vec<Line> {
    aoc::parsing::lines_to_vec::<Line>(input.as_ref()).expect("to have correct aoc input")
}

fn intersect(a: &(i64, i64), b: &(i64, i64)) -> Option<(i64, i64)> {
    if (a.0 <= b.1 && b.0 <= a.1) || (a.1.abs_diff(b.0) <= 1) || (b.1.abs_diff(a.0) <= 1) {
        return Some((a.0.min(b.0), a.1.max(b.1)));
    }

    return None;
}

fn part1(input: &Vec<Line>, y_level: i64) -> String {
    let beacon_xs = input
        .into_iter()
        .filter(|line| line.b.y == y_level)
        .map(|line| line.b.x)
        .unique()
        .collect::<Vec<i64>>();

    return input
        .into_iter()
        .filter(|line| (line.s.y - y_level).abs() <= line.dist)
        .map(|line| {
            let spread = line.dist - (line.s.y - y_level).abs();

            return (line.s.x - spread, line.s.x + spread);
        })
        .sorted()
        .fold(Vec::new(), |mut acc, int| {
            if acc.is_empty() {
                return vec![int];
            } else if let Some(int) = intersect(acc.last().unwrap(), &int) {
                *acc.last_mut().unwrap() = int;

                return acc;
            } else {
                acc.push(int);

                return acc;
            }
        })
        .into_iter()
        .map(|int| {
            let mut total = int.1 - int.0 + 1;

            for x in &beacon_xs {
                if int.0 <= *x && x <= &int.1 {
                    total -= 1;
                }
            }

            return total;
        })
        .sum::<i64>()
        .to_string();
}

fn yeet(input: &Vec<Line>, y_level: i64) -> Option<i64> {
    let input = input
        .into_iter()
        .filter(|line| (line.s.y - y_level).abs() <= line.dist)
        .map(|line| {
            let spread = line.dist - (line.s.y - y_level).abs();

            return (line.s.x - spread, line.s.x + spread);
        })
        .sorted()
        .fold(Vec::new(), |mut acc, int| {
            if acc.is_empty() {
                return vec![int];
            } else if let Some(int) = intersect(acc.last().unwrap(), &int) {
                *acc.last_mut().unwrap() = int;

                return acc;
            } else {
                acc.push(int);

                return acc;
            }
        });

    if input.len() == 1 {
        return None;
    }

    return Some(input.first()?.1 + 1);
}

fn part2(input: &Vec<Line>, y_level: i64) -> String {
    for row in 0..=y_level {
        if let Some(col) = yeet(input, row) {
            println!("{}, {}", row, col);
            return (4000000 * col + row).to_string();
        }
        // 13673971349056
    }

    "".to_owned()
}

fn main() {
    let input = parse_input(aoc::input::read_from_stdin());

    println!("Part 1: {}", part1(&input, Y_LEVEL_1));
    println!("Part 2: {}", part2(&input, Y_LEVEL_2));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example1() {
        let input = super::parse_input(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        );

        assert_eq!(super::part1(&input, 10), "26");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        );

        assert_eq!(super::part2(&input, 20), "56000011");
    }
}
