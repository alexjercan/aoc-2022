use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").ok_or(Self::Err::ParseError(
            "expect to have 2 numbers sep by ','".to_string(),
        ))?;

        let x = x
            .parse::<usize>()
            .map_err(|e| Self::Err::ParseError(e.to_string()))?;
        let y = y
            .parse::<usize>()
            .map_err(|e| Self::Err::ParseError(e.to_string()))?;

        return Ok(Point::new(x, y));
    }
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        return Point { x, y };
    }

    fn span(self: &Self, other: &Self) -> Vec<Self> {
        let min_x = self.x.min(other.x);
        let max_x = self.x.max(other.x);
        let min_y = self.y.min(other.y);
        let max_y = self.y.max(other.y);

        let mut span = Vec::new();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                span.push(Point::new(x, y));
            }
        }

        return span;
    }
}

#[derive(Debug, Clone)]
struct Line {
    points: Vec<Point>,
}

impl FromStr for Line {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split(" -> ")
            .map(|p| p.parse::<Point>())
            .collect::<Result<_, _>>()?;

        return Ok(Line { points });
    }
}

impl Into<Vec<Point>> for Line {
    fn into(self) -> Vec<Point> {
        let mut points = Vec::new();

        for i in 0..self.points.len() - 1 {
            let mut span = self.points[i].span(&self.points[i + 1]);
            points.append(&mut span);
        }

        return points;
    }
}

fn parse_input(input: impl AsRef<str>) -> Vec<Line> {
    aoc::parsing::lines_to_vec::<Line>(input.as_ref()).expect("to have correct aoc input")
}

#[derive(Debug, Clone)]
struct World {
    rocks: HashSet<Point>,
    sands: HashSet<Point>,
}

impl World {
    fn new(rocks: HashSet<Point>) -> Self {
        Self {
            rocks,
            sands: HashSet::new(),
        }
    }

    fn spawn_part1(self: &mut Self) -> Option<Point> {
        let max_y = self.rocks.clone().into_iter().map(|p| p.y).max()?;

        let mut p = Point::new(500, 0);

        loop {
            if p.y + 1 > max_y {
                return None;
            }

            let pn = Point::new(p.x, p.y + 1);
            if !self.rocks.contains(&pn) && !self.sands.contains(&pn)  {
                p.y += 1;
                continue;
            }

            let pn = Point::new(p.x - 1, p.y + 1);
            if !self.rocks.contains(&pn) && !self.sands.contains(&pn)  {
                p.x -= 1;
                p.y += 1;
                continue;
            }

            let pn = Point::new(p.x + 1, p.y + 1);
            if !self.rocks.contains(&pn) && !self.sands.contains(&pn)  {
                p.x += 1;
                p.y += 1;
                continue;
            }

            self.sands.insert(p);
            return Some(p);
        }
    }

    fn spawn_part2(self: &mut Self) -> Option<Point> {
        let max_y = self.rocks.clone().into_iter().map(|p| p.y).max()?;
        let floor_y = max_y + 2;

        let mut p = Point::new(500, 0);

        loop {
            if p.y + 1 >= floor_y {
                self.sands.insert(p);
                return Some(p);
            }

            let pn = Point::new(p.x, p.y + 1);
            if !self.rocks.contains(&pn) && !self.sands.contains(&pn)  {
                p.y += 1;
                continue;
            }

            let pn = Point::new(p.x - 1, p.y + 1);
            if !self.rocks.contains(&pn) && !self.sands.contains(&pn)  {
                p.x -= 1;
                p.y += 1;
                continue;
            }

            let pn = Point::new(p.x + 1, p.y + 1);
            if !self.rocks.contains(&pn) && !self.sands.contains(&pn)  {
                p.x += 1;
                p.y += 1;
                continue;
            }

            self.sands.insert(p);
            return Some(p);
        }
    }
}

impl From<Vec<Line>> for World {
    fn from(value: Vec<Line>) -> Self {
        let rocks = value.into_iter().fold(HashSet::new(), |mut acc, line| {
            let points: Vec<Point> = line.into();
            acc.extend(points);
            return acc;
        });

        return World::new(rocks);
    }
}

impl ToString for World {
    fn to_string(&self) -> String {
        let (min_x, max_x, min_y, max_y) = self.rocks.clone().into_iter().fold(
            (usize::MAX, usize::MIN, usize::MAX, usize::MIN),
            |(min_x, max_x, min_y, max_y), p| {
                (
                    min_x.min(p.x),
                    max_x.max(p.x),
                    min_y.min(p.y),
                    max_y.max(p.y),
                )
            },
        );

        let min_y = 0;

        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;

        let map = self.rocks.clone().into_iter().fold(vec![vec!['.'; width]; height], |mut acc, p| {
            acc[p.y - min_y][p.x - min_x] = '#';
            return acc;
        });

        let mut map = self.sands.clone().into_iter().fold(map, |mut acc, p| {
            if p.y > min_y && p.x > min_x && p.y - min_y < height && p.x - min_x < width {
                acc[p.y - min_y][p.x - min_x] = 'o';
            }
            return acc;
        });

        map[0][500 - min_x] = '+';

        return map
            .into_iter()
            .map(|line| line.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
    }
}

fn part1(input: &Vec<Line>) -> String {
    let mut world: World = input.to_vec().into();
    while let Some(_) = world.spawn_part1() {}
    return world.sands.len().to_string();
}

fn part2(input: &Vec<Line>) -> String {
    let mut world: World = input.to_vec().into();
    while let Some(p) = world.spawn_part2() {
        if p.x == 500 && p.y == 0 {
            break;
        }
    }
    return world.sands.len().to_string();
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
            "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
        );

        assert_eq!(super::part1(&input), "24");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
        );

        assert_eq!(super::part2(&input), "93");
    }
}
