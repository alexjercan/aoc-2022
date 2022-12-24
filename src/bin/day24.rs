use pathfinding::prelude::astar;
use std::collections::BTreeMap;

const DIR_5: [(i32, i32); 5] = [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Dir {
    type Error = aoc::error::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Dir::Up),
            '>' => Ok(Dir::Right),
            'v' => Ok(Dir::Down),
            '<' => Ok(Dir::Left),
            _ => Err(Self::Error::ParseError("error".to_string())),
        }
    }
}

impl Into<char> for Dir {
    fn into(self) -> char {
        match self {
            Dir::Up => '^',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Right => '>',
        }
    }
}

impl Dir {
    fn step(self: &Self, pos: (i32, i32)) -> (i32, i32) {
        (
            pos.0 + DIR_5[*self as usize].0,
            pos.1 + DIR_5[*self as usize].1,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    blizzards: BTreeMap<(i32, i32), Vec<Dir>>,
    width: i32,
    height: i32,
}

impl Map {
    fn valid(self: &Self, pos: (i32, i32)) -> bool {
        return 0 < pos.0
            && pos.0 < self.width as i32
            && 0 < pos.1
            && pos.1 < self.height as i32
            && !self.blizzards.contains_key(&pos);
    }

    fn tick(self: &Self) -> Self {
        let n_blizzards: BTreeMap<(i32, i32), Vec<Dir>> =
            self.blizzards
                .iter()
                .fold(BTreeMap::new(), |mut acc, (pos, bs)| {
                    bs.iter().for_each(|b| {
                        let mut n_pos = b.step(*pos);
                        if n_pos.0 < 1 {
                            n_pos.0 = self.width - 1;
                        }
                        if n_pos.1 < 1 {
                            n_pos.1 = self.height - 1;
                        }
                        if n_pos.0 > self.width - 1 {
                            n_pos.0 = 1;
                        }
                        if n_pos.1 > self.height - 1 {
                            n_pos.1 = 1;
                        }

                        acc.entry(n_pos)
                            .and_modify(|e| e.push(*b))
                            .or_insert(vec![*b]);
                    });

                    return acc;
                });

        return Map {
            blizzards: n_blizzards,
            width: self.width,
            height: self.height,
        };
    }
}

impl ToString for Map {
    fn to_string(&self) -> String {
        (1..self.height)
            .map(|y| {
                (1..self.width)
                    .map(|x| {
                        if let Some(cell) = self.blizzards.get(&(x as i32, y as i32)) {
                            if cell.len() > 1 {
                                return cell
                                    .len()
                                    .to_string()
                                    .chars()
                                    .into_iter()
                                    .next()
                                    .unwrap_or('?');
                            } else {
                                return cell[0].into();
                            }
                        } else {
                            return '.';
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug)]
struct Input {
    map: Map,
    source: (i32, i32),
    dest: (i32, i32),
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

    let blizzards = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                Some(((x as i32, y as i32), vec![Dir::try_from(c).ok()?]))
            })
        })
        .collect::<BTreeMap<(i32, i32), Vec<Dir>>>();

    return Input {
        map: Map {
            blizzards,
            width: width as i32,
            height: height as i32,
        },
        source: (source as i32, 0),
        dest: (dest as i32, height as i32),
    };
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    map: Map,
    pos: (i32, i32),
    source: (i32, i32),
    dest: (i32, i32),
}

fn successors(state: &State) -> Vec<(State, i32)> {
    let n_map = state.map.tick();

    return DIR_5
        .iter()
        .filter_map(|&d| {
            let n_pos = (state.pos.0 + d.0, state.pos.1 + d.1);

            if n_map.valid(n_pos) || n_pos == state.dest || n_pos == state.source {
                return Some((
                    State {
                        map: n_map.clone(),
                        pos: n_pos,
                        source: state.source,
                        dest: state.dest,
                    },
                    1,
                ));
            } else {
                return None;
            }
        })
        .collect();
}

fn heuristic(state: &State) -> i32 {
    return (state.dest.0 - state.pos.0).abs() + (state.dest.1 - state.pos.1).abs();
}

fn success(state: &State) -> bool {
    state.dest == state.pos
}

fn part1(input: &Input) -> String {
    let start = State {
        map: input.map.clone(),
        pos: input.source,
        source: input.source,
        dest: input.dest,
    };

    let (_, cost) = astar(&start, successors, heuristic, success)
        .expect("to have a solution");

    return cost.to_string();
}

fn part2(input: &Input) -> String {
    "".to_owned()
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

    // #[test]
    fn part2_example1() {
        let input = super::parse_input("");

        assert_eq!(super::part2(&input), "");
    }
}
