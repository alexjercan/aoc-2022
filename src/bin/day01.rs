fn parse_input(input: impl AsRef<str>) -> Vec<u32> {
    let mut calories = input
        .as_ref()
        .trim_end()
        .split("\n\n")
        .map(|elf| {
            aoc::parsing::lines_to_vec::<u32>(elf)
                .unwrap()
                .into_iter()
                .sum()
        })
        .collect::<Vec<u32>>();

    calories.sort();
    calories.reverse();

    return calories;
}

fn solve(calories: &Vec<u32>, top: usize) -> u32 {
    calories.into_iter().take(top).sum()
}

fn part1(calories: &Vec<u32>) -> String {
    return solve(calories, 1).to_string();
}

fn part2(calories: &Vec<u32>) -> String {
    return solve(calories, 3).to_string();
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
        let input = super::parse_input(String::from(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        ));

        assert_eq!(super::part1(&input), "24000");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(String::from(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        ));

        assert_eq!(super::part2(&input), "45000");
    }
}
