use itertools::Itertools;
use pathfinding::prelude::astar;
use std::collections::HashSet;

type Point = (i32, i32);

const DIR_5: [Point; 5] = [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)];
const DIR_C: [char; 4] = ['^', 'v', '<', '>'];

fn step(pos: Point, dir: Point) -> Point {
    (pos.0 + dir.0, pos.1 + dir.1)
}

#[derive(Debug, Clone)]
struct Map {
    blizzards: Vec<HashSet<Point>>,
    width: i32,
    height: i32,
}

impl Map {
    fn blizzard(self: &Self, x: i32, y: i32) -> char {
        let bs: Vec<char> = self
            .blizzards
            .iter()
            .enumerate()
            .filter_map(|(dir, bs)| {
                if bs.contains(&(x, y)) {
                    return Some(DIR_C[dir]);
                } else {
                    return None;
                }
            })
            .collect();

        if bs.len() == 0 {
            return '.';
        }

        if bs.len() == 1 {
            return bs[0];
        }

        return bs
            .len()
            .to_string()
            .chars()
            .into_iter()
            .next()
            .unwrap_or('?');
    }
}

impl ToString for Map {
    fn to_string(&self) -> String {
        (1..self.height)
            .map(|y| {
                (1..self.width)
                    .map(|x| {
                        return self.blizzard(x as i32, y as i32);
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug)]
struct Input {
    blizzards: Vec<Vec<HashSet<(i32, i32)>>>,
    width: i32,
    height: i32,
    source: (i32, i32),
    dest: (i32, i32),
}

fn ticks(
    blizzard: &HashSet<(i32, i32)>,
    dir: Point,
    width: i32,
    height: i32,
) -> Vec<HashSet<Point>> {
    let mut blizzards = vec![blizzard.clone()];
    loop {
        let n_blizzard = blizzards[blizzards.len() - 1]
            .iter()
            .map(|&pos| {
                let mut n_pos = step(pos, dir);
                if n_pos.0 < 1 {
                    n_pos.0 = width - 1;
                }
                if n_pos.1 < 1 {
                    n_pos.1 = height - 1;
                }
                if n_pos.0 > width - 1 {
                    n_pos.0 = 1;
                }
                if n_pos.1 > height - 1 {
                    n_pos.1 = 1;
                }

                return n_pos;
            })
            .collect();

        if n_blizzard == blizzards[0] {
            break;
        }

        blizzards.push(n_blizzard);
    }

    return blizzards;
}

fn parse_input(input: impl AsRef<str>) -> Input {
    let lines: Vec<_> = input.as_ref().lines().collect();

    let height = lines.len() - 1;
    let width = lines[0].len() - 1;

    let source = lines[0]
        .chars()
        .position(|c| c == '.')
        .expect("to have an entry point");
    let dest = lines[height]
        .chars()
        .position(|c| c == '.')
        .expect("to have an exit point");

    let blizzards: Vec<HashSet<(i32, i32)>> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let dir = DIR_C.iter().position(|&d| d == c)?;
                return Some((dir, (x as i32, y as i32)));
            })
        })
        .into_group_map()
        .into_iter()
        .sorted()
        .map(|(_, v)| HashSet::from_iter(v.into_iter()))
        .collect();

    let blizzards: Vec<_> = (0..DIR_C.len())
        .into_iter()
        .map(|dir| ticks(&blizzards[dir], DIR_5[dir], width as i32, height as i32))
        .collect();

    return Input {
        blizzards,
        width: width as i32,
        height: height as i32,
        source: (source as i32, 0),
        dest: (dest as i32, height as i32),
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: (i32, i32),
    minute: usize,
}

fn valid(input: &Input, state: &State) -> bool {
    0 < state.pos.0
        && state.pos.0 < input.width as i32
        && 0 < state.pos.1
        && state.pos.1 < input.height as i32
        && input
            .blizzards
            .iter()
            .all(|bs| !bs[state.minute % bs.len()].contains(&state.pos))
}

fn successors(input: &Input, state: &State) -> Vec<(State, i32)> {
    DIR_5
        .iter()
        .filter_map(|&d| {
            let n_state = State {
                pos: (state.pos.0 + d.0, state.pos.1 + d.1),
                minute: state.minute + 1,
            };

            if valid(input, &n_state) || n_state.pos == input.dest || n_state.pos == input.source {
                return Some((n_state, 1));
            } else {
                return None;
            }
        })
        .collect()
}

fn manhattan(a: Point, b: Point) -> i32 {
    return (a.0 - b.0).abs() + (a.1 - b.1).abs();
}

fn part1(input: &Input) -> String {
    let state = State { pos: input.source, minute: 0 };

    let (_, cost) = astar(&state,
        |s| successors(input, s),
        |s| manhattan(s.pos, input.dest),
        |s| s.pos == input.dest,
    ).expect("to have solution");

    return cost.to_string();
}

fn part2(input: &Input) -> String {
    let state = State { pos: input.source, minute: 0 };

    let (states, cost1) = astar(&state,
        |s| successors(input, s),
        |s| manhattan(s.pos, input.dest),
        |s| s.pos == input.dest,
    ).expect("to have solution");

    let state = State { pos: input.dest, minute: states[states.len() - 1].minute };
    let (states, cost2) = astar(&state,
        |s| successors(input, s),
        |s| manhattan(s.pos, input.source),
        |s| s.pos == input.source,
    ).expect("to have solution");

    let state = State { pos: input.source, minute: states[states.len() - 1].minute };
    let (_, cost3) = astar(&state,
        |s| successors(input, s),
        |s| manhattan(s.pos, input.dest),
        |s| s.pos == input.dest,
    ).expect("to have solution");

    return (cost1 + cost2 + cost3).to_string();
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
            "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
        );

        assert_eq!(super::part1(&input), "18");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
        );


        assert_eq!(super::part2(&input), "54");
    }
}
