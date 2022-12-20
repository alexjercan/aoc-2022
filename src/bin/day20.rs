fn parse_input(input: impl AsRef<str>) -> Vec<isize> {
    input
        .as_ref()
        .lines()
        .map(|line| line.parse::<isize>())
        .collect::<Result<_, _>>()
        .expect("to have correct aoc input")
}

fn mix(xs: &mut Vec<(usize, isize)>, order: &Vec<(usize, isize)>) {
    for x in order {
        let i = xs.iter().position(|y| x == y).unwrap();
        let im = (i as isize + x.1).rem_euclid((xs.len() - 1) as isize) as usize;

        xs.remove(i);
        xs.insert(im, *x);
    }
}

fn part1(input: &Vec<isize>) -> String {
    let mut xs: Vec<(isize, bool)> = input.into_iter().map(|x| (*x, false)).collect();

    let mut i = 0;

    while i < xs.len() {
        match xs[i] {
            (_, true) => i = i + 1,
            (x, false) => {
                let im = (i as isize + x).rem_euclid((xs.len() - 1) as isize) as usize;

                xs.remove(i);
                xs.insert(im, (x, true));

                if im < i {
                    i = i + 1;
                }
            }
        }
    }

    let xs = xs.iter().map(|(x, _)| *x).collect::<Vec<isize>>();
    let zero = xs.iter().position(|x| *x == 0).expect("to have at least one zero in input");

    return [1000, 2000, 3000].iter().fold(0, |acc, i| {
            acc + xs[(zero + i).rem_euclid(xs.len())]
    }).to_string();
}

fn part2(input: &Vec<isize>) -> String {
    let input: Vec<(usize, isize)> = input.into_iter().enumerate().map(|x| (x.0, x.1 * 811589153)).collect();
    let mut xs = input.clone();

    for _ in 0..10 {
        mix(&mut xs, &input);
    }

    let zero = xs.iter().position(|x| (*x).1 == 0).expect("to have at least one zero in input");

    return [1000, 2000, 3000].iter().fold(0, |acc, i| {
            acc + xs[(zero + i).rem_euclid(xs.len())].1
    }).to_string();
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
        let input = super::parse_input("1
2
-3
3
-2
0
4");

        assert_eq!(super::part1(&input), "3");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input("1
2
-3
3
-2
0
4");

        assert_eq!(super::part2(&input), "1623178306");
    }
}
