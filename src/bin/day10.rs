use std::str::FromStr;

#[derive(Debug)]
enum Instr {
    Noop,
    Addx(i32),
}

impl FromStr for Instr {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            return Ok(Instr::Noop);
        } else if s.starts_with("addx") {
            let (_, value) = s.split_once(' ').ok_or_else(|| {
                Self::Err::ParseError("expected addx to have on argument: {s}".to_string())
            })?;
            let value = value
                .parse::<i32>()
                .map_err(|e| Self::Err::ParseError(e.to_string()))?;
            return Ok(Instr::Addx(value));
        } else {
            return Err(Self::Err::ParseError(
                "unexpected instruction {s}".to_string(),
            ));
        }
    }
}

impl Instr {
    fn cycles(self: &Self) -> usize {
        match self {
            Instr::Noop => 1,
            Instr::Addx(_) => 2,
        }
    }
}

fn parse_input(input: impl AsRef<str>) -> Vec<Instr> {
    aoc::parsing::lines_to_vec::<Instr>(input).expect("aoc to give correct input")
}

fn part1(input: &Vec<Instr>) -> String {
    let mut total_cycles = 0;
    let mut sum = 0;
    let mut x = 1;

    for instr in input {
        let cycles = instr.cycles();

        for _ in 0..cycles {
            total_cycles += 1;

            if (total_cycles == 20) || (total_cycles > 20 && (total_cycles - 20) % 40 == 0) {
                sum += x * total_cycles;
            }
        }

        match instr {
            Instr::Noop => (),
            Instr::Addx(v) => x += v,
        }
    }

    return sum.to_string();
}

fn part2(input: &Vec<Instr>) -> String {
    let mut total_cycles: usize = 0;
    let mut x: i32 = 1;

    let mut crt = vec![vec!['.'; 40]; 6];

    for instr in input {
        let cycles = instr.cycles();

        for _ in 0..cycles {
            let crt_row: usize = total_cycles / 40;
            let crt_col: usize = total_cycles % 40;

            if (x.abs() as usize).abs_diff(crt_col) <= 1 {
                crt[crt_row][crt_col] = '#';
            }

            total_cycles += 1;
        }

        match instr {
            Instr::Noop => (),
            Instr::Addx(v) => x += v,
        }
    }

    return crt
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
}

fn main() {
    let input = parse_input(aoc::input::read_from_stdin());

    println!("Part 1: {}", part1(&input));
    println!("Part 2: \n{}", part2(&input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example1() {
        let input = super::parse_input(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        );

        assert_eq!(super::part1(&input), "13140");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        );

        assert_eq!(
            super::part2(&input),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
