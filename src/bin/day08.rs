use take_until::TakeUntilExt;

fn parse_input(input: impl AsRef<str>) -> Vec<Vec<u8>> {
    input
        .as_ref()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("to only have numbers in the input") as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

fn is_visible(map: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let line_len = map[y].len();
    if x == 0 || x == line_len - 1 {
        return true;
    }

    let len = map.len();
    if y == 0 || y == len - 1 {
        return true;
    }

    let height = map[x][y];

    return (0..y).fold(true, |acc, j| acc && height > map[x][j])
        || (y + 1..len).fold(true, |acc, j| acc && height > map[x][j])
        || (0..x).fold(true, |acc, i| acc && height > map[i][y])
        || (x + 1..line_len).fold(true, |acc, i| acc && height > map[i][y]);
}

fn scenic_score(map: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let line_len = map[y].len();
    if x == 0 || x == line_len - 1 {
        return 0;
    }

    let len = map.len();
    if y == 0 || y == len - 1 {
        return 0;
    }

    let height = map[x][y];

    return map[x][0..y]
        .into_iter()
        .rev()
        .take_until(|&e| e >= &height)
        .count()
        * map[x][y + 1..len]
            .into_iter()
            .take_until(|&e| e >= &height)
            .count()
        * map[0..x]
            .into_iter()
            .map(|line| line[y])
            .rev()
            .take_until(|&e| e >= height)
            .count()
        * map[x + 1..line_len]
            .into_iter()
            .map(|line| line[y])
            .take_until(|&e| e >= height)
            .count();
}

fn part1(input: &Vec<Vec<u8>>) -> String {
    let mut count = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if is_visible(input, x, y) {
                count += 1;
            }
        }
    }

    return count.to_string();
}

fn part2(input: &Vec<Vec<u8>>) -> String {
    let mut best = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let score = scenic_score(input, x, y);
            if score > best {
                best = score;
            }
        }
    }

    return best.to_string();
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
            "30373
25512
65332
33549
35390",
        );

        assert_eq!(super::part1(&input), "21");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "30373
25512
65332
33549
35390",
        );

        assert_eq!(super::part2(&input), "8");
    }
}
