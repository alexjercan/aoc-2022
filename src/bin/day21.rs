use std::collections::HashMap;

#[derive(Debug)]
enum Operation<'a> {
    Number(isize),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

impl<'a> TryFrom<&'a str> for Operation<'a> {
    type Error = aoc::error::Error;

    fn try_from(s: &'a str) -> Result<Operation<'a>, <Operation<'a> as TryFrom<&'a str>>::Error> {
        if let Some(num) = s.parse::<isize>().ok() {
            return Ok(Operation::Number(num));
        } else if let Some((lhs, rhs)) = s.split_once(" + ") {
            return Ok(Operation::Add(lhs, rhs));
        } else if let Some((lhs, rhs)) = s.split_once(" - ") {
            return Ok(Operation::Sub(lhs, rhs));
        } else if let Some((lhs, rhs)) = s.split_once(" * ") {
            return Ok(Operation::Mul(lhs, rhs));
        } else if let Some((lhs, rhs)) = s.split_once(" / ") {
            return Ok(Operation::Div(lhs, rhs));
        } else {
            unreachable!()
        }
    }
}

type Input<'a> = HashMap<&'a str, Operation<'a>>;

fn parse_input<'a>(input: &'a str) -> Input<'a> {
    input
        .lines()
        .map(|line| {
            if let Some((lhs, rhs)) = line.split_once(": ") {
                if let Some(op) = Operation::try_from(rhs).ok() {
                    return Some((lhs, op));
                }
            }

            return None;
        })
        .collect::<Option<HashMap<_, _>>>()
        .expect("to have correct aoc input")
}

fn dfs(node: &str, input: &Input) -> isize {
    let op = input.get(node).expect("to have an entry for each node");

    match op {
        Operation::Number(num) => *num,
        Operation::Add(lhs, rhs) => dfs(lhs, input) + dfs(rhs, input),
        Operation::Sub(lhs, rhs) => dfs(lhs, input) - dfs(rhs, input),
        Operation::Mul(lhs, rhs) => dfs(lhs, input) * dfs(rhs, input),
        Operation::Div(lhs, rhs) => dfs(lhs, input) / dfs(rhs, input),
    }
}

fn part1(input: &Input) -> String {
    dfs("root", input).to_string()
}

fn reverse_answer(answer: isize, node: &str, input: &Input) -> Option<isize> {
    if node == "humn" {
        return Some(answer);
    }

    let op = input.get(node).expect("to have an entry for each node");

    let (lhs, rhs) = match op {
        Operation::Number(_) => return None,
        Operation::Add(lhs, rhs) => (lhs, rhs),
        Operation::Sub(lhs, rhs) => (lhs, rhs),
        Operation::Mul(lhs, rhs) => (lhs, rhs),
        Operation::Div(lhs, rhs) => (lhs, rhs),
    };

    let rhs_ans = dfs(rhs, input);
    let lhs_humn = match op {
        Operation::Number(_) => unreachable!(),
        Operation::Add(lhs, _) => reverse_answer(answer - rhs_ans, lhs, input),
        Operation::Sub(lhs, _) => reverse_answer(answer + rhs_ans, lhs, input),
        Operation::Mul(lhs, _) => reverse_answer(answer / rhs_ans, lhs, input),
        Operation::Div(lhs, _) => reverse_answer(answer * rhs_ans, lhs, input),
    };

    let lhs_ans = dfs(lhs, input);
    let rhs_humn = match op {
        Operation::Number(_) => unreachable!(),
        Operation::Add(_, rhs) => reverse_answer(answer - lhs_ans, rhs, input),
        Operation::Sub(_, rhs) => reverse_answer(lhs_ans - answer, rhs, input),
        Operation::Mul(_, rhs) => reverse_answer(answer / lhs_ans, rhs, input),
        Operation::Div(_, rhs) => reverse_answer(rhs_ans / answer, rhs, input),
    };

    return lhs_humn.or(rhs_humn);
}

fn part2(input: &Input) -> String {
    let root = input.get("root").expect("to have the root monkey");

    let (lhs, rhs) = match root {
        Operation::Number(_) => unreachable!(),
        Operation::Add(lhs, rhs) => (lhs, rhs),
        Operation::Sub(lhs, rhs) => (lhs, rhs),
        Operation::Mul(lhs, rhs) => (lhs, rhs),
        Operation::Div(lhs, rhs) => (lhs, rhs),
    };

    let lhs_ans = dfs(lhs, input);
    let rhs_ans = dfs(rhs, input);

    return reverse_answer(rhs_ans, lhs, input)
        .or(reverse_answer(lhs_ans, rhs, input))
        .expect("one of them to work")
        .to_string();
}

fn main() {
    let input = aoc::input::read_from_stdin();
    let input = parse_input(&input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example1() {
        let input = super::parse_input(
            "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32",
        );

        assert_eq!(super::part1(&input), "152");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32",
        );

        assert_eq!(super::part2(&input), "301");
    }
}
