use std::{collections::VecDeque, str::FromStr};

#[derive(Debug, Clone)]
enum Operation {
    Multi(usize),
    Add(usize),
    Square,
}

impl FromStr for Operation {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, op) = s.split_once('=').ok_or(Self::Err::ParseError(
            "operation should have an = sign".to_string(),
        ))?;
        let op = op.trim();

        if op.contains('+') {
            let (_, rhs) = op.split_once('+').ok_or(Self::Err::ParseError(
                "operation should have a + sign".to_string(),
            ))?;

            let rhs = rhs
                .trim()
                .parse::<usize>()
                .map_err(|e| Self::Err::ParseError(e.to_string()))?;

            return Ok(Operation::Add(rhs));
        } else if op.contains('*') {
            let (_, rhs) = op.split_once('*').ok_or(Self::Err::ParseError(
                "operation should have a * sign".to_string(),
            ))?;

            if let Some(rhs) = rhs.trim().parse::<usize>().ok() {
                return Ok(Operation::Multi(rhs));
            } else {
                return Ok(Operation::Square);
            }
        } else {
            return Err(Self::Err::ParseError("invalid operation".to_string()));
        }
    }
}

impl Operation {
    fn apply(self: &Self, value: usize) -> usize {
        match self {
            Operation::Multi(v) => v * value,
            Operation::Add(v) => v + value,
            Operation::Square => value * value,
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    divisor: usize,
    true_index: usize,
    false_index: usize,
}

impl FromStr for Test {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().into_iter();

        let divisor = lines
            .next()
            .ok_or(Self::Err::ParseError(
                "should have a line for test".to_string(),
            ))?
            .split(' ')
            .last()
            .ok_or(Self::Err::ParseError(
                "condition should be correct".to_string(),
            ))?
            .parse::<usize>()
            .map_err(|e| Self::Err::ParseError(e.to_string()))?;

        let true_index = lines
            .next()
            .ok_or(Self::Err::ParseError(
                "should have a line for true index".to_string(),
            ))?
            .split(' ')
            .last()
            .ok_or(Self::Err::ParseError(
                "should have items for true index".to_string(),
            ))?
            .parse::<usize>()
            .map_err(|e| Self::Err::ParseError(e.to_string()))?;
        let false_index = lines
            .next()
            .ok_or(Self::Err::ParseError(
                "should have a line for false index".to_string(),
            ))?
            .split(' ')
            .last()
            .ok_or(Self::Err::ParseError(
                "should have items for false index".to_string(),
            ))?
            .parse::<usize>()
            .map_err(|e| Self::Err::ParseError(e.to_string()))?;

        return Ok(Test {
            divisor,
            true_index,
            false_index,
        });
    }
}

impl Test {
    fn test(self: &Self, value: usize) -> usize {
        if value % self.divisor == 0 {
            return self.true_index;
        } else {
            return self.false_index;
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,

    inspections: usize,
}

impl FromStr for Monkey {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();

        let (_, items) = lines[1]
            .split_once(':')
            .ok_or(Self::Err::ParseError("should have : for items".to_string()))?;
        let items = items
            .trim()
            .split(", ")
            .map(|item| {
                item.parse::<usize>()
                    .map_err(|e| Self::Err::ParseError(e.to_string()))
            })
            .collect::<Result<VecDeque<usize>, Self::Err>>()?;

        let operation = lines[2].parse::<Operation>()?;

        let test = lines[3..].join("\n").parse::<Test>()?;

        return Ok(Monkey {
            items,
            operation,
            test,

            inspections: 0,
        });
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.inspections == other.inspections
    }
}

impl Eq for Monkey {}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inspections.partial_cmp(&other.inspections)
    }
}

impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inspections.cmp(&other.inspections)
    }
}

impl Monkey {
    fn act1(self: &mut Self) -> Option<(usize, usize)> {
        if let Some(item) = self.items.pop_front() {
            let item = self.operation.apply(item) / 3;
            let next = self.test.test(item);

            self.inspections += 1;

            return Some((item, next));
        }

        return None;
    }

    fn act2(self: &mut Self, divisor: usize) -> Option<(usize, usize)> {
        if let Some(item) = self.items.pop_front() {
            let item = self.operation.apply(item) % divisor;
            let next = self.test.test(item);

            self.inspections += 1;

            return Some((item, next));
        }

        return None;
    }

    fn round1(self: &mut Self) -> Vec<(usize, usize)> {
        let mut actions = Vec::new();

        while let Some(act) = self.act1() {
            actions.push(act);
        }

        return actions;
    }

    fn round2(self: &mut Self, divisor: usize) -> Vec<(usize, usize)> {
        let mut actions = Vec::new();

        while let Some(act) = self.act2(divisor) {
            actions.push(act);
        }

        return actions;
    }

    fn give(self: &mut Self, item: usize) {
        self.items.push_back(item);
    }
}

fn parse_input(input: impl AsRef<str>) -> Vec<Monkey> {
    input
        .as_ref()
        .split("\n\n")
        .map(|m| m.parse::<Monkey>())
        .collect::<Result<Vec<Monkey>, aoc::error::Error>>()
        .expect("to have correct aoc input")
}

fn part1(input: &Vec<Monkey>) -> String {
    let mut monkeys = input.to_vec();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let actions = monkeys[i].round1();

            for (item, next) in actions {
                monkeys[next].give(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.cmp(a));

    return (monkeys[0].inspections * monkeys[1].inspections).to_string();
}

fn part2(input: &Vec<Monkey>) -> String {
    let mut monkeys = input.to_vec();

    let common_div = input.into_iter().fold(1, |acc, m| acc * m.test.divisor);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let actions = monkeys[i].round2(common_div);

            for (item, next) in actions {
                monkeys[next].give(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.cmp(a));

    return (monkeys[0].inspections * monkeys[1].inspections).to_string();
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
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        );

        assert_eq!(super::part1(&input), "10605");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        );

        assert_eq!(super::part2(&input), "2713310158");
    }
}
