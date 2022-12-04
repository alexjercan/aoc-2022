use std::collections::HashMap;

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
        .map(|r| {
            for i in 0..r.len() / 2 {
                for j in r.len() / 2..r.len() {
                    if r[i] == r[j] {
                        return r[i];
                    }
                }
            }

            return 0;
        })
        .sum::<usize>()
        .to_string()
}

fn part2(input: &Vec<Vec<usize>>) -> String {
    input
        .chunks(3)
        .flat_map(|g| {
            g.into_iter()
                .flat_map(aoc::collections::unique)
                .fold(HashMap::new(), |mut acc, c| {
                    acc.entry(c).and_modify(|e| *e += 1).or_insert(1);
                    acc
                })
                .into_iter()
                .find_map(|(k, v)| if v == 3 { Some(k) } else { None })
                .ok_or(0)
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
