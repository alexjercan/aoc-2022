use glam::{ivec2, IVec2};

fn parse_input(input: impl AsRef<str>) -> Vec<IVec2> {
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

fn propose(position: IVec2, others: &Vec<IVec2>, dirs: [Dir; 4]) -> Option<IVec2> {
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

    if others.iter().all(|pos| !n_positions.contains(pos)) {
        return None;
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

        if others.iter().all(|pos| !n_positions.contains(pos)) {
            return Some(n_positions[1]);
        }
    }

    return None;
}

fn round(elfs: &Vec<IVec2>, dirs: [Dir; 4]) -> Vec<IVec2> {
    let props: Vec<_> = elfs
        .iter()
        .map(|&position| propose(position, elfs, dirs))
        .collect();

    return props
        .iter()
        .enumerate()
        .map(|(i, &prop)| match prop {
            Some(position) => {
                if props
                    .iter()
                    .enumerate()
                    .all(|(j, prop)| i == j || prop.is_none() || prop.unwrap() != position)
                {
                    return position;
                } else {
                    return elfs[i];
                }
            }
            None => elfs[i],
        })
        .collect();
}

fn part1(input: &Vec<IVec2>) -> String {
    let mut elfs = input.clone();
    let mut dirs = [Dir::North, Dir::South, Dir::West, Dir::East];

    for _ in 0..10 {
        elfs = round(&elfs, dirs);

        let used = dirs[0];
        for i in 0..3 {
            dirs[i] = dirs[i+1];
        }
        dirs[3] = used;
    }

    let (min_x, max_x) = elfs
        .iter()
        .fold((i32::MAX, i32::MIN), |(mut min, mut max), elf| {
            min = min.min(elf.x);
            max = max.max(elf.x);

            return (min, max);
        });
    let (min_y, max_y) = elfs
        .iter()
        .fold((i32::MAX, i32::MIN), |(mut min, mut max), elf| {
            min = min.min(elf.y);
            max = max.max(elf.y);

            return (min, max);
        });


    return ((max_x - min_x + 1) * (max_y - min_y + 1) - elfs.len() as i32).to_string();
}

fn part2(input: &Vec<IVec2>) -> String {
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
