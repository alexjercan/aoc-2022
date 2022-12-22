use glam::{ivec2, ivec3, vec3, IVec2, Mat3};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub const ROT_XY: [Mat3; 4] = [
    Mat3::from_cols_array(&[0.0, 0.0, 1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0]),
    Mat3::from_cols_array(&[1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0]),
    Mat3::from_cols_array(&[0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0]),
    Mat3::from_cols_array(&[1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0]),
];

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
}

type Map = HashMap<(i32, i32), Tile>;

fn parse_map(input: impl AsRef<str>) -> Map {
    input
        .as_ref()
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, char)| match char {
                    '.' => Some(((row as i32, col as i32), Tile::Open)),
                    '#' => Some(((row as i32, col as i32), Tile::Wall)),
                    _ => None,
                })
        })
        .collect()
}

#[derive(Debug)]
enum Rotation {
    Left,
    Right,
}

#[derive(Debug)]
enum Move {
    Step(i32),
    Rotate(Rotation),
}

fn parse_moves(input: impl AsRef<str>) -> Option<Vec<Move>> {
    input
        .as_ref()
        .chars()
        .into_iter()
        .group_by(|c| c.is_alphabetic())
        .into_iter()
        .map(|(_, g)| {
            let g = g.collect::<String>();
            match g {
                _ if g == "R" => Some(Move::Rotate(Rotation::Right)),
                _ if g == "L" => Some(Move::Rotate(Rotation::Left)),
                digits => Some(Move::Step(digits.parse::<i32>().ok()?)),
            }
        })
        .collect()
}

type Input = (Map, Vec<Move>);

fn parse_input(input: impl AsRef<str>) -> Input {
    let (map, moves) = input
        .as_ref()
        .split_once("\n\n")
        .expect("to have correct aoc input");

    return (
        parse_map(map.trim_end()),
        parse_moves(moves.trim_end()).expect("to have correct set of moves"),
    );
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

fn dir_to_delta(dir: &Dir) -> (i32, i32) {
    match dir {
        Dir::Right => (0, 1),
        Dir::Down => (1, 0),
        Dir::Left => (0, -1),
        Dir::Up => (-1, 0),
    }
}

fn dir_rotate(dir: &Dir, r: &Rotation) -> Dir {
    match r {
        Rotation::Left => match dir {
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Up => Dir::Left,
        },
        Rotation::Right => match dir {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
        },
    }
}

type State = (i32, i32, Dir);

fn apply_move1(m: &Move, map: &Map, (row, col, dir): &State) -> State {
    match m {
        Move::Step(n) => {
            let (d_row, d_col) = dir_to_delta(dir);

            let mut prev_row = *row;
            let mut prev_col = *col;

            for _ in 0..*n {
                let (n_row, n_col) = (prev_row + d_row, prev_col + d_col);

                match map.get(&(n_row, n_col)) {
                    Some(Tile::Open) => {
                        prev_row = n_row;
                        prev_col = n_col;
                    }
                    Some(Tile::Wall) => break,
                    None => {
                        let mut p_row = prev_row;
                        let mut p_col = prev_col;

                        while map.get(&(p_row, p_col)).is_some() {
                            p_row = p_row - d_row;
                            p_col = p_col - d_col;
                        }

                        let (n_row, n_col) = (p_row + d_row, p_col + d_col);
                        match map.get(&(n_row, n_col)) {
                            Some(Tile::Open) => {
                                prev_row = n_row;
                                prev_col = n_col;
                            }
                            Some(Tile::Wall) => break,
                            None => unreachable!(),
                        }
                    }
                }
            }

            return (prev_row, prev_col, *dir);
        }
        Move::Rotate(r) => return (*row, *col, dir_rotate(dir, r)),
    }
}

fn part1((map, moves): &Input) -> String {
    let start_row = 0;
    let start_col = *map
        .iter()
        .filter_map(|((row, col), tile)| {
            if *row == start_row && *tile == Tile::Open {
                Some(col)
            } else {
                None
            }
        })
        .min()
        .expect("to have the start col");
    let start_dir = Dir::Right;
    let mut state: State = (start_row, start_col, start_dir);

    for m in moves {
        state = apply_move1(m, map, &state);
    }

    let (row, col, dir) = state;
    return (1000 * (row + 1) + 4 * (col + 1) + dir as i32).to_string();
}

fn part2((map, moves): &Input) -> String {
    // convert map to use IVec2
    let map = map
        .iter()
        .map(|(k, v)| (ivec2(k.1, k.0), v))
        .collect::<HashMap<_, _>>();

    // width and height of the shape
    let (w, h) = map.iter().fold((0, 0), |mut acc, (&vec, _)| {
        acc.0 = acc.0.max(vec.x);
        acc.1 = acc.1.max(vec.y);
        return acc;
    });

    // side of one square
    let square_size = f64::sqrt((map.len() / 6) as _) as i32;

    // indices of the squares that build the cube
    let mut skeleton: HashSet<IVec2> = (0..=w / square_size)
        .flat_map(|row| {
            (0..=h / square_size)
                .filter_map(|col| {
                    if map.contains_key(&ivec2(row * square_size, col * square_size)) {
                        Some(ivec2(row, col))
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>()
        })
        .collect();

    // top left face
    let face = ivec2((0..).find(|&x| skeleton.contains(&ivec2(x, 0))).unwrap(), 0);
    skeleton.remove(&face);

    // Planet surface positions mapped back to 2D chart.
    let mut cube_chart = HashMap::new();

    let mut search = vec![(face, Mat3::IDENTITY)];
    while let Some((face, m)) = search.pop() {
        for (x, y) in (0..square_size)
            .flat_map(|x| (0..square_size).map(|y| (x, y)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
        {
            let chart_pos = face * square_size + ivec2(x, y);

            // Project to (slightly above) unit cube surface.
            // Sample cell centers so add the 0.5s
            let mut p3 = vec3(x as f32 + 0.5, y as f32 + 0.5, -0.5);
            p3 = (p3 / square_size as f32) - 0.5;

            // Transform to current face;
            p3 = m * p3;

            // Project back to regular space.
            p3 += 0.5;
            p3 *= square_size as f32;
            // Cell center correction.
            p3 -= 0.5;

            let p3 = p3.round().as_ivec3();

            // This part is tricky, floating point artifacts can mess up even cover.
            cube_chart.insert(p3, chart_pos);
        }

        // Continue building cube faces while there are unmapped sectors.
        //
        // Multiplying the transformation matrix along chart traversal keeps
        // track of the 3D space frame.
        for dir in 0..4 {
            let f = face + [ivec2(1, 0), ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)][dir];
            if skeleton.contains(&f) {
                search.push((f, m * ROT_XY[dir]));
                skeleton.remove(&f);
            }
        }
    }

    // Start out standing on top face.
    let mut prev_pos = ivec3(0, 0, -1);
    // Facing right
    let mut prev_dir = ivec3(1, 0, 0);
    // With the current up vector.
    let mut prev_up = ivec3(0, 0, -1);

    for m in moves {
        match m {
            Move::Rotate(Rotation::Left) => prev_dir = prev_dir.cross(-prev_up),
            Move::Rotate(Rotation::Right) => prev_dir = prev_dir.cross(prev_up),
            Move::Step(n) => {
                for _ in 0..*n {
                    let mut n_pos = prev_pos + prev_dir;
                    let mut n_dir = prev_dir;
                    let mut n_up = prev_up;

                    if !cube_chart.contains_key(&n_pos) {
                        // We walked off the face.
                        // New direction points downwards from old frame.
                        n_dir = -prev_up;
                        // And the new face has the same normal as the direction we
                        // were walking before.
                        n_up = prev_dir;

                        // Step along the new dir to get back on surface.
                        n_pos += n_dir;
                    }

                    if map.get(&cube_chart[&n_pos]).unwrap() == &&Tile::Wall {
                        break;
                    } else {
                        prev_pos = n_pos;
                        prev_dir = n_dir;
                        prev_up = n_up;
                    }
                }
            }
        }
    }

    let chart_pos = cube_chart[&prev_pos];

    // Reconstruct facing.
    let facing_vec = if let Some(&p2) = cube_chart.get(&(prev_pos + prev_dir)) {
        // Either next position is on chart...
        p2 - chart_pos
    } else {
        // ...or the previous one is.
        chart_pos - cube_chart[&(prev_pos - prev_dir)]
    };
    let facing = [ivec2(1, 0), ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)]
        .iter()
        .position(|&p| p == facing_vec)
        .unwrap() as i32;

    return (4 * (chart_pos.x + 1) + 1000 * (chart_pos.y + 1) + facing).to_string();
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
            "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5",
        );

        assert_eq!(super::part1(&input), "6032");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5",
        );

        assert_eq!(super::part2(&input), "5031");
    }
}
