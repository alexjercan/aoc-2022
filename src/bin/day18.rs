use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        return Point { x, y, z };
    }

    fn neighbors(self: &Self) -> Vec<Self> {
        return [
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .map(|(i, j, k)| Point::new(self.x + i, self.y + j, self.z + k))
        .to_vec();
    }

    fn bounded(self: &Self, min: isize, max: isize) -> bool {
        return min <= self.x
            && self.x <= max
            && min <= self.y
            && self.y <= max
            && min <= self.z
            && self.z <= max;
    }
}

impl FromStr for Point {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(3, ',').into_iter();

        let x = iter
            .next()
            .ok_or(Self::Err::ParseError("x".to_string()))?
            .parse::<isize>()
            .map_err(|e| Self::Err::ParseError(e.to_string()))?;
        let y = iter
            .next()
            .ok_or(Self::Err::ParseError("y".to_string()))?
            .parse::<isize>()
            .map_err(|e| Self::Err::ParseError(e.to_string()))?;
        let z = iter
            .next()
            .ok_or(Self::Err::ParseError("z".to_string()))?
            .parse::<isize>()
            .map_err(|e| Self::Err::ParseError(e.to_string()))?;

        return Ok(Point::new(x, y, z));
    }
}

fn parse_input(input: impl AsRef<str>) -> HashSet<Point> {
    return HashSet::from_iter(
        aoc::parsing::lines_to_vec::<Point>(input.as_ref()).expect("to have correct aoc input"),
    );
}

fn part1(input: &HashSet<Point>) -> String {
    return input
        .iter()
        .flat_map(|p| p.neighbors())
        .filter(|s| !input.contains(s))
        .count()
        .to_string();
}

fn part2(input: &HashSet<Point>) -> String {
    let max = input
        .iter()
        .flat_map(|p| [p.x, p.y, p.z])
        .max()
        .expect("at least 1 point")
        + 1;

    let mut visited = HashSet::new();
    let mut q: Vec<Point> = vec![Point::new(0, 0, 0)];

    while let Some(p) = q.pop() {
        for s in p.neighbors() {
            if !visited.contains(&s) && !input.contains(&s) && s.bounded(-1, max) {
                visited.insert(s);
                q.push(s);
            }
        }
    }

    return input
        .iter()
        .flat_map(|p| p.neighbors())
        .filter(|s| visited.contains(s))
        .count()
        .to_string();
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
            "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5",
        );

        assert_eq!(super::part1(&input), "64");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5",
        );

        assert_eq!(super::part2(&input), "58");
    }
}
