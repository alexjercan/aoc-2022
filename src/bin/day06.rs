use std::collections::HashSet;

fn parse_input(input: impl AsRef<str>) -> Vec<char> {
    return input.as_ref().chars().collect::<Vec<char>>();
}

fn solve(input: &Vec<char>, window: usize) -> usize {
    for i in 0..input.len() - window {
        let s = &input[i..i + window].iter().collect::<HashSet<_>>();

        if s.len() == window {
            return i + window;
        }
    }

    0
}

fn part1(input: &Vec<char>) -> String {
    return solve(input, 4).to_string();
}

fn part2(input: &Vec<char>) -> String {
    return solve(input, 14).to_string();
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
        let input = super::parse_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb");

        assert_eq!(super::part1(&input), "7");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb");

        assert_eq!(super::part2(&input), "19");
    }
}
