use std::str::FromStr;

const TOTAL_SPACE: usize = 70000000;
const UPDATE_SPACE: usize = 30000000;

#[derive(Debug, Clone)]
enum Node {
    Directory(String),
    File(String, usize),
}

impl FromStr for Node {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s
            .split_once(' ')
            .ok_or(Self::Err::ParseError("failed to split line".to_string()))?;

        return Ok(match lhs {
            "dir" => Node::Directory(rhs.to_string()),
            lhs => Node::File(
                rhs.to_string(),
                lhs.parse()
                    .map_err(|_| Self::Err::ParseError(format!("failed to parse size {}", lhs)))?,
            ),
        });
    }
}

fn parse_input(input: impl AsRef<str>) -> Vec<Vec<Node>> {
    input
        .as_ref()
        .split("$ ls\n")
        .map(|r| {
            r.lines()
                .filter_map(|line| line.parse::<Node>().ok())
                .collect::<Vec<Node>>()
        })
        .skip(1)
        .collect::<Vec<Vec<Node>>>()
}

fn dfs(node: Node, walk: &mut Vec<Vec<Node>>, result: &mut Vec<usize>) -> usize {
    match node {
        Node::File(_, size) => size,
        Node::Directory(_) => {
            let nodes = walk.remove(0);

            let size = nodes
                .into_iter()
                .map(|node| dfs(node, walk, result))
                .sum::<usize>();
            result.push(size);

            return size;
        }
    }
}

fn part1(input: &Vec<Vec<Node>>) -> String {
    let root = Node::Directory("/".to_string());
    let mut walk = input.to_vec();
    let mut result = Vec::new();

    _ = dfs(root, &mut walk, &mut result);

    result
        .into_iter()
        .filter(|&size| size <= 100000)
        .sum::<usize>()
        .to_string()
}

fn part2(input: &Vec<Vec<Node>>) -> String {
    let root = Node::Directory("/".to_string());
    let mut walk = input.to_vec();
    let mut result = Vec::new();

    let size = dfs(root, &mut walk, &mut result);

    let available_space = TOTAL_SPACE - size;
    if available_space >= UPDATE_SPACE {
        panic!("enough space for update");
    }

    let required_space = UPDATE_SPACE - available_space;

    result.sort();

    for size in result {
        if size >= required_space {
            return size.to_string();
        }
    }

    unreachable!();
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
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        );

        assert_eq!(super::part1(&input), "95437");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input(
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        );

        assert_eq!(super::part2(&input), "24933642");
    }
}
