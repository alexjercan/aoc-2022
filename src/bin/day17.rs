use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
enum Push {
    Left,
    Right,
}

impl FromStr for Push {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => return Ok(Push::Left),
            ">" => return Ok(Push::Right),
            c => {
                return Err(Self::Err::ParseError(format!(
                    "expect one of < or > but found '{}'",
                    c
                )))
            }
        }
    }
}

impl Push {
    fn apply(self: &Self, x: i64) -> i64 {
        match self {
            Self::Left => x - 1,
            Self::Right => x + 1,
        }
    }
}

fn parse_input(input: impl AsRef<str>) -> Vec<Push> {
    input
        .as_ref()
        .trim()
        .split("")
        .filter(|s| !s.is_empty())
        .map(str::parse::<Push>)
        .collect::<Result<_, _>>()
        .expect("to have correct aoc input")
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        return Point { x, y };
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.y.partial_cmp(&other.y);
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.y.cmp(&other.y);
    }
}

#[derive(Debug, Clone)]
struct Piece {
    points: Vec<Point>,
}

impl Piece {
    fn new_horizontal(p: Point) -> Piece {
        Piece {
            points: (0..4)
                .map(|x| Point::new(p.x + x, p.y))
                .collect::<Vec<Point>>(),
        }
    }

    fn new_plus(p: Point) -> Piece {
        let mut points = (0..3)
            .map(|x| Point::new(p.x + x, p.y + 1))
            .collect::<Vec<_>>();
        points.push(Point::new(p.x + 1, p.y));
        points.push(Point::new(p.x + 1, p.y + 2));

        return Piece { points };
    }

    fn new_l(p: Point) -> Piece {
        let mut points = (0..3).map(|x| Point::new(p.x + x, p.y)).collect::<Vec<_>>();
        points.push(Point::new(p.x + 2, p.y + 1));
        points.push(Point::new(p.x + 2, p.y + 2));

        return Piece { points };
    }

    fn new_vertical(p: Point) -> Piece {
        Piece {
            points: (0..4)
                .map(|y| Point::new(p.x, p.y + y))
                .collect::<Vec<Point>>(),
        }
    }

    fn new_square(p: Point) -> Piece {
        let mut points = (0..2).map(|x| Point::new(p.x + x, p.y)).collect::<Vec<_>>();
        points.push(Point::new(p.x, p.y + 1));
        points.push(Point::new(p.x + 1, p.y + 1));

        return Piece { points };
    }

    fn collision(self: &Self, map: &HashSet<Point>) -> bool {
        for p in &self.points {
            let pn = Point::new(p.x, p.y - 1);
            if map.contains(&pn) {
                return true;
            }
        }

        return false;
    }

    fn gravity(self: &mut Self) {
        for p in &mut self.points {
            p.y -= 1;
        }
    }

    fn push(self: &mut Self, map: &HashSet<Point>, push: &Push) {
        if (&self.points).into_iter().all(|p| {
            let new_x = push.apply(p.x);
            let pn = Point::new(new_x, p.y);

            return 0 <= new_x && new_x < 7 && !map.contains(&pn);
        }) {
            for p in &mut self.points {
                p.x = push.apply(p.x);
            }
        }
    }
}

#[derive(Debug)]
struct Spawner {
    piece_counter: usize,
    tick_counter: usize,
    map: HashSet<Point>,
    top: i64,

    heights: Vec<i64>,
}

impl ToString for Spawner {
    fn to_string(&self) -> String {
        let min_x = 0;
        let max_x = 6;
        let min_y = 0;
        let max_y = self.top + 10;

        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;

        let map = (&self.map)
            .into_iter()
            .fold(vec![vec!['.'; width]; height], |mut acc, p| {
                acc[p.y as usize][p.x as usize] = '#';
                return acc;
            });

        return map
            .into_iter()
            .map(|line| line.into_iter().collect::<String>())
            .rev()
            .collect::<Vec<String>>()
            .join("\n");
    }
}

impl Spawner {
    fn new() -> Self {
        let floor = (0..7).map(|x| Point::new(x, 0)).collect::<HashSet<_>>();
        return Spawner {
            piece_counter: 0,
            tick_counter: 0,
            map: floor,
            top: 0,
            heights: Vec::new(),
        };
    }

    fn spawn(self: &mut Self) -> Piece {
        let p = Point::new(2, self.top + 4);
        let cnt = self.piece_counter;

        self.piece_counter = (self.piece_counter + 1) % 5;

        match cnt {
            0 => Piece::new_horizontal(p),
            1 => Piece::new_plus(p),
            2 => Piece::new_l(p),
            3 => Piece::new_vertical(p),
            4 => Piece::new_square(p),
            _ => unreachable!(),
        }
    }

    fn drop(self: &mut Self, input: &Vec<Push>) {
        let mut piece = self.spawn();

        self.tick_counter = self.tick_counter % input.len();

        loop {
            piece.push(&self.map, &input[self.tick_counter]);
            self.tick_counter = (self.tick_counter + 1) % input.len();

            if piece.collision(&self.map) {
                break;
            }
            piece.gravity();
        }

        let prev = self.top;
        for p in &piece.points {
            if self.top < p.y {
                self.top = p.y;
            }
        }
        self.heights.push(self.top - prev);

        self.map.extend(piece.points);
    }

    fn find_cycle(self: &Self) -> Option<(usize, usize)> {
        let heights = &self.heights;
        for offset in 0..heights.len() {
            for window in 1..(heights.len() - offset) / 2 {
                if heights[offset..]
                    .into_iter()
                    .zip(heights[offset..offset + window].into_iter().cycle())
                    .all(|(a, b)| a == b)
                {
                    return Some((offset, window));
                }
            }
        }

        return None;
    }
}

fn part1(input: &Vec<Push>) -> String {
    let mut spawner = Spawner::new();

    for _ in 0..2022 {
        spawner.drop(input);
    }

    return spawner.top.to_string();
}

fn part2(input: &Vec<Push>) -> String {
    let mut spawner = Spawner::new();

    for _ in 0..5000 {
        spawner.drop(input);
    }

    if let Some((offset, window)) = spawner.find_cycle() {
        const DROPS: usize = 1000000000000;
        let init = spawner.heights[..offset].into_iter().sum::<i64>();
        let cycle = spawner.heights[offset..offset + window]
            .into_iter()
            .sum::<i64>();

        let cycles = ((DROPS - offset) / window) as i64;
        let end = (DROPS - offset) % window;
        let end = spawner.heights[offset..offset + end]
            .into_iter()
            .sum::<i64>();

        return (init + cycles * cycle + end).to_string();
    }

    return "sad noises".to_string();
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
        let input = super::parse_input(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");

        assert_eq!(super::part1(&input), "3068");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");

        assert_eq!(super::part2(&input), "1514285714288");
    }
}
