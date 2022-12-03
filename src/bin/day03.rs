use std::collections::HashSet;

fn item_to_value(c: char) -> usize {
    if 'A' <= c && c <= 'Z' {
        return c as usize - 'A' as usize + 1 + 26;
    } else {
        return c as usize - 'a' as usize + 1;
    }
}

fn parse_input(input: impl AsRef<str>) -> Vec<Vec<usize>> {
    input
        .as_ref()
        .lines()
        .map(|line| line.chars().map(item_to_value).collect::<Vec<usize>>())
        .collect()
}

fn part1(input: &Vec<Vec<usize>>) -> String {
    input
        .into_iter()
        .flat_map(|r| {
            let sets = r
                .chunks(r.len() / 2)
                .map(|c| HashSet::from_iter(c.iter().cloned()))
                .collect::<Vec<HashSet<usize>>>();

            sets.iter()
                .skip(1)
                .fold(sets[0].clone(), |acc, hs| {
                    acc.intersection(hs).cloned().collect()
                })
                .iter()
                .next()
                .cloned()
        })
        .sum::<usize>()
        .to_string()
}

fn part2(input: &Vec<Vec<usize>>) -> String {
    input
        .chunks(3)
        .flat_map(|g| {
            let sets = g
                .iter()
                .map(|c| HashSet::from_iter(c.iter().cloned()))
                .collect::<Vec<HashSet<usize>>>();

            sets.iter()
                .skip(1)
                .fold(sets[0].clone(), |acc, hs| {
                    acc.intersection(hs).cloned().collect()
                })
                .iter()
                .next()
                .cloned()
        })
        .sum::<usize>()
        .to_string()
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
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        assert_eq!(super::part1(&input), "157");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        assert_eq!(super::part2(&input), "70");
    }
}
