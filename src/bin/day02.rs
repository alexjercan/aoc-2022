#[derive(Debug)]
enum Error {
    ParseError(String)
}

#[derive(Debug)]
struct Play {
    elf: u8,
    player: u8,
}

impl std::str::FromStr for Play {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (elf, player) = s.split_once(' ').ok_or_else(|| Error::ParseError(s.to_owned()))?;

        let elf: char = elf.chars().next().ok_or_else(|| Error::ParseError(elf.to_owned()))?;
        let player: char = player.chars().next().ok_or_else(|| Error::ParseError(player.to_owned()))?;

        return Ok(Play {
            elf: elf as u8 - 'A' as u8 + 1,
            player: player as u8 - 'X' as u8 + 1,
        });
    }
}

fn parse_input(input: impl AsRef<str>) -> Vec<Play> {
    aoc::parsing::lines_to_vec::<_, Play>(input).unwrap()
}

fn strategy1(play: &Play) -> u32 {
    if (play.elf == 3 && play.player == 1) || (play.elf + 1 == play.player) {
        return 6 + play.player as u32;
    } else if play.elf == play.player {
        return 3 + play.player as u32;
    } else {
        return play.player as u32;
    }
}

fn part1(input: &Vec<Play>) -> String {
    return input.into_iter().map(strategy1).sum::<u32>().to_string();
}

fn strategy2(play: &Play) -> u32 {
    if play.player == 1 {
        return ((play.elf as i32 - 2).rem_euclid(3) + 1) as u32;
    } else if play.player == 2 {
        return 3 + play.elf as u32;
    } else {
        return 6 + (play.elf as u32 % 3) + 1;
    }
}

fn part2(input: &Vec<Play>) -> String {
    return input.into_iter().map(strategy2).sum::<u32>().to_string();
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
        let input = super::parse_input("A Y
B X
C Z");

        assert_eq!(super::part1(&input), "15");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input("A Y
B X
C Z");

        assert_eq!(super::part2(&input), "12");
    }
}
