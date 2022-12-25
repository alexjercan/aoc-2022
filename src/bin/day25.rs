fn snafu_to_digit(snafu: char) -> Option<i64> {
    match snafu {
        '2' => Some(2),
        '1' => Some(1),
        '0' => Some(0),
        '-' => Some(-1),
        '=' => Some(-2),
        _ => None,
    }
}

fn digit_to_snafu(digit: i64) -> Option<char> {
    match digit {
        2 => Some('2'),
        1 => Some('1'),
        0 => Some('0'),
        -1 => Some('-'),
        -2 => Some('='),
        _ => None,
    }
}

fn snafu_to_i64(snafu: impl AsRef<str>) -> Option<i64> {
    Some(
        snafu
            .as_ref()
            .chars()
            .map(snafu_to_digit)
            .collect::<Option<Vec<_>>>()?
            .iter()
            .rev()
            .fold((0, 1), |(num, base), d| {
                (num + base * d, base * 5)
            })
            .0,
    )
}

fn i64_to_snafu(num: i64) -> Option<String> {
    let mut num = num;
    let mut digits = Vec::new();
    while num > 0 {
        let digit = num % 5;
        num = num / 5;

        digits.push(digit);
    }

    for i in 0..digits.len() - 1 {
        if digits[i] >= 3{
            digits[i] -= 5;
            digits[i+1] += 1;
        }
    }

    let n = digits.len() - 1;
    if digits[n] >= 3 {
        digits[n] -= 5;
        digits.push(1);
    }

    return digits.into_iter().rev().map(digit_to_snafu).collect::<Option<String>>();
}

fn parse_input(input: impl AsRef<str>) -> Vec<i64> {
    input
        .as_ref()
        .lines()
        .map(snafu_to_i64)
        .collect::<Option<_>>()
        .expect("to have correct aoc input")
}

fn part1(input: &Vec<i64>) -> String {
    return i64_to_snafu(input.iter().sum()).expect("to work");
}

fn main() {
    let input = parse_input(aoc::input::read_from_stdin());

    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example1() {
        let input = super::parse_input(
            "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122",
        );

        assert_eq!(super::part1(&input), "2=-1=0");
    }
}
