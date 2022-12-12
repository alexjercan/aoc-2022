use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

#[derive(Debug)]
struct Map {
    map: HashMap<(usize, usize), usize>,
    width: usize,
    height: usize,
    src: (usize, usize),
    dest: (usize, usize),
}

impl FromStr for Map {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut map = HashMap::new();
        let mut src = (0, 0);
        let mut dest = (0, 0);

        let height = lines.len();
        let width = lines[0].len();

        for x in 0..lines.len() {
            for y in 0..lines[x].len() {
                if lines[x][y] == 'S' {
                    src = (x, y);
                    lines[x][y] = 'a';
                } else if lines[x][y] == 'E' {
                    dest = (x, y);
                    lines[x][y] = 'z';
                }

                map.insert((x, y), lines[x][y] as usize - 'a' as usize);
            }
        }

        return Ok(Map {
            map,
            width,
            height,
            src,
            dest,
        });
    }
}

fn parse_input(input: impl AsRef<str>) -> Map {
    input
        .as_ref()
        .parse::<Map>()
        .expect("to have correct input")
}

fn neighbors(map: &Map, node: (usize, usize)) -> Vec<(usize, usize)> {
    let edges = &map.map;
    let height = map.height as isize;
    let width = map.width as isize;

    let value = edges.get(&node).unwrap();

    let (x, y) = node;

    return [(0, 1), (0, -1), (-1, 0), (1, 0)]
        .into_iter()
        .filter_map(|(i, j)| {
            let xn = x as isize + i;
            let yn = y as isize + j;

            if xn < 0 || xn >= height || yn < 0 || yn >= width {
                return None;
            }

            let xn = xn as usize;
            let yn = yn as usize;

            let valuen = edges.get(&(xn, yn)).unwrap();

            if value + 1 >= *valuen {
                return Some((xn, yn));
            } else {
                return None;
            }
        })
        .collect();
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(map: &Map) -> Option<usize> {
    let start = map.src;
    let goal = map.dest;
    let edges = &map.map;

    let mut dist: HashMap<_, _> = edges.keys().map(|&k| (k, usize::MAX)).collect();

    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > *dist.get(&position).unwrap() {
            continue;
        }

        for edge in neighbors(map, position) {
            if cost + 1 < *dist.get(&edge).unwrap() {
                heap.push(State {
                    cost: cost + 1,
                    position: edge,
                });
                dist.insert(edge, cost + 1);
            }
        }
    }

    return None;
}

fn part1(input: &Map) -> String {
    return shortest_path(input).unwrap().to_string();
}

fn part2(input: &Map) -> String {
    return input
        .map
        .iter()
        .filter(|(_, &v)| return v == 0)
        .filter_map(|(&k, _)| {
            let test_input = Map {
                map: input.map.clone(),
                width: input.width,
                height: input.height,
                src: k,
                dest: input.dest,
            };

            return shortest_path(&test_input);
        })
        .min()
        .unwrap()
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
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        );

        assert_eq!(super::part1(&input), "31");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        );

        assert_eq!(super::part2(&input), "29");
    }
}
