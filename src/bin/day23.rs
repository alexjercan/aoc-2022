use std::collections::{HashSet, HashMap};

use glam::{ivec2, IVec2};

fn parse_input(input: impl AsRef<str>) -> HashSet<IVec2> {
    input
        .as_ref()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => Some(ivec2(x as i32, y as i32)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    South,
    West,
    East,
}

fn propose(position: IVec2, others: &HashSet<IVec2>, dirs: [Dir; 4]) -> IVec2 {
    let n_positions = [
        position + ivec2(-1, -1),
        position + ivec2(0, -1),
        position + ivec2(1, -1),
        position + ivec2(-1, 1),
        position + ivec2(0, 1),
        position + ivec2(1, 1),
        position + ivec2(-1, 0),
        position + ivec2(1, 0),
    ];

    if n_positions.iter().all(|n_pos| !others.contains(n_pos)) {
        return position;
    }

    for dir in &dirs {
        let n_positions = match dir {
            Dir::North => [
                position + ivec2(-1, -1),
                position + ivec2(0, -1),
                position + ivec2(1, -1),
            ],
            Dir::South => [
                position + ivec2(-1, 1),
                position + ivec2(0, 1),
                position + ivec2(1, 1),
            ],
            Dir::West => [
                position + ivec2(-1, -1),
                position + ivec2(-1, 0),
                position + ivec2(-1, 1),
            ],
            Dir::East => [
                position + ivec2(1, -1),
                position + ivec2(1, 0),
                position + ivec2(1, 1),
            ],
        };

        if n_positions.iter().all(|n_pos| !others.contains(n_pos)) {
            return n_positions[1];
        }
    }

    return position;
}

fn round(elves: &HashSet<IVec2>, dirs: [Dir; 4]) -> HashSet<IVec2> {
    elves.iter().map(|&elf| {
        (propose(elf, elves, dirs), elf)
    }).fold(HashMap::<IVec2, Vec<IVec2>>::new(), |mut acc, (prop, elf)| {
        acc.entry(prop).and_modify(|e| e.push(elf)).or_insert(vec![elf]);

        return acc;
    }).iter().flat_map(|(&prop, elves)| {
        if elves.len() == 1 {
            return vec![prop];
        } else {
            return elves.to_owned();
        }
    }).collect()
}

fn part1(input: &HashSet<IVec2>) -> String {
    let mut elves = input.clone();
    let mut dirs = [Dir::North, Dir::South, Dir::West, Dir::East];

    for _ in 0..10 {
        elves = round(&elves, dirs);

        let used = dirs[0];
        for i in 0..3 {
            dirs[i] = dirs[i+1];
        }
        dirs[3] = used;
    }

    let (min_x, max_x) = elves
        .iter()
        .fold((i32::MAX, i32::MIN), |(mut min, mut max), elf| {
            min = min.min(elf.x);
            max = max.max(elf.x);

            return (min, max);
        });
    let (min_y, max_y) = elves
        .iter()
        .fold((i32::MAX, i32::MIN), |(mut min, mut max), elf| {
            min = min.min(elf.y);
            max = max.max(elf.y);

            return (min, max);
        });


    return ((max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32).to_string();
}

fn part2(input: &HashSet<IVec2>) -> String {
    let mut elves = input.clone();
    let mut dirs = [Dir::North, Dir::South, Dir::West, Dir::East];

    for index in 1..usize::MAX {
        let n_elves = round(&elves, dirs);

        if n_elves == elves {
            return index.to_string();
        }

        let used = dirs[0];
        for i in 0..3 {
            dirs[i] = dirs[i+1];
        }
        dirs[3] = used;

        elves = n_elves;
    }

    return "no way you waited this long".to_owned();
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
            "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..",
        );

        assert_eq!(super::part1(&input), "110");
    }

    // #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..",
        );

        assert_eq!(super::part2(&input), "20");
    }
}
