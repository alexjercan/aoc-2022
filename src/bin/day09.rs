use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
enum Move {
    Up(isize),
    Left(isize),
    Right(isize),
    Down(isize),
}

impl FromStr for Move {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, steps) = s
            .split_once(' ')
            .expect("the input to be of the form 'D int'");

        let steps = steps
            .parse::<isize>()
            .expect("the input to have in the second position an int");

        match dir {
            "U" => Ok(Move::Up(steps)),
            "L" => Ok(Move::Left(steps)),
            "R" => Ok(Move::Right(steps)),
            "D" => Ok(Move::Down(steps)),
            _ => Err(Self::Err::ParseError("unexpected direction".to_owned())),
        }
    }
}

fn parse_input(input: impl AsRef<str>) -> Vec<Move> {
    aoc::parsing::lines_to_vec::<Move>(input.as_ref()).expect("aoc to give correct input")
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn apply_move(self: &mut Self, m: &Move) {
        match m {
            Move::Up(steps) => {
                self.y += steps;
            }
            Move::Left(steps) => {
                self.x -= steps;
            }
            Move::Right(steps) => {
                self.x += steps;
            }
            Move::Down(steps) => {
                self.y -= steps;
            }
        }
    }

    fn is_touching(self: &Self, other: &Self) -> bool {
        return self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1;
    }

    fn step_follow(self: &mut Self, target: &Self, points: &mut HashSet<Point>) {
        let x_error = target.x - self.x;
        let y_error = target.y - self.y;

        let x_sign = x_error.signum();
        let y_sign = y_error.signum();

        self.x += x_sign;
        self.y += y_sign;

        points.insert(self.clone());
    }
}

fn part1(input: &Vec<Move>) -> String {
    let mut map: HashSet<Point> = HashSet::new();
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);

    map.insert(tail.clone());

    for m in input {
        head.apply_move(m);

        while !tail.is_touching(&head) {
            tail.step_follow(&head, &mut map);
        }
    }

    return map.len().to_string();
}

fn part2(input: &Vec<Move>) -> String {
    let mut maps: Vec<HashSet<Point>> = vec![HashSet::new(); 10];
    let mut knots: Vec<Point> = vec![Point::new(0, 0); 10];

    for (map, knot) in maps.iter_mut().zip(knots.iter()) {
        map.insert(knot.clone());
    }

    for m in input {
        let head = &mut knots[0];

        head.apply_move(m);

        let mut running = true;
        while running {
            for i in 1..knots.len() {
                let head = &knots[i-1].clone();
                let tail = &mut knots[i];

                if !tail.is_touching(&head) {
                    tail.step_follow(&head, &mut maps[i]);
                }
            }

            running = (1..knots.len()).any(|i| {
                let head = &knots[i-1];
                let tail = &knots[i];

                return !tail.is_touching(&head);
            });
        }
    }

    return maps.into_iter().last().unwrap().len().to_string();
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
        let input = super::parse_input("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");

        assert_eq!(super::part1(&input), "13");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");

        assert_eq!(super::part2(&input), "36");
    }
}
