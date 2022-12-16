use std::{collections::HashMap, str::FromStr};

struct Pair {
    name: String,
    node: (usize, Vec<String>),
}

impl FromStr for Pair {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ').into_iter();

        tokens.next();

        let name = tokens
            .next()
            .ok_or(Self::Err::ParseError(
                "expect token with name of node".to_string(),
            ))?
            .to_string();

        tokens.next();
        tokens.next();

        let flow = tokens
            .next()
            .ok_or(Self::Err::ParseError(
                "expect token with flow value".to_string(),
            ))?
            .split_once('=')
            .ok_or(Self::Err::ParseError(
                "expect flow value to have = sign".to_string(),
            ))?
            .1
            .split_once(';')
            .ok_or(Self::Err::ParseError(
                "expect token with flow value to end with ; sign".to_string(),
            ))?
            .0
            .parse::<usize>()
            .map_err(|e| Self::Err::ParseError(e.to_string()))?;

        tokens.next();
        tokens.next();
        tokens.next();
        tokens.next();

        let neighbors = tokens
            .collect::<String>()
            .split(',')
            .map(String::from)
            .collect();

        return Ok(Pair {
            name,
            node: (flow, neighbors),
        });
    }
}

#[derive(Debug)]
struct Input {
    mapping: Vec<String>,
    dists: Vec<Vec<usize>>,
    flows: Vec<usize>,
    start: usize,
}

fn shortest_paths(edges: &HashMap<String, (usize, Vec<String>)>) -> Input {
    let mut non_zero: Vec<String> = edges
        .into_iter()
        .filter_map(|(k, (flow, _))| {
            if *flow == 0 {
                return None;
            } else {
                return Some(k.clone());
            }
        })
        .collect();
    non_zero.insert(0, "AA".to_string());

    let mut dists: HashMap<_, HashMap<_, _>> = edges
        .clone()
        .into_keys()
        .map(|k| {
            (
                k,
                edges.clone().into_keys().map(|k| (k, usize::MAX)).collect(),
            )
        })
        .collect();

    for (u, (_, vs)) in edges {
        for v in vs {
            let d = dists.get_mut(u).unwrap().get_mut(v).unwrap();
            *d = 1;
        }
    }

    for (u, _) in edges {
        let d = dists.get_mut(u).unwrap().get_mut(u).unwrap();
        *d = 0;
    }

    for (k, _) in edges {
        for (i, _) in edges {
            for (j, _) in edges {
                if let Some(dn) = dists
                    .get(i)
                    .unwrap()
                    .get(k)
                    .unwrap()
                    .checked_add(*dists.get(k).unwrap().get(j).unwrap())
                {
                    let d = dists.get_mut(i).unwrap().get_mut(j).unwrap();
                    if *d > dn {
                        *d = dn;
                    }
                }
            }
        }
    }

    let dists = dists
        .into_iter()
        .filter_map(|(u, map)| {
            if !non_zero.contains(&&u) {
                return None;
            }

            let dist = map
                .into_iter()
                .filter_map(|(v, d)| {
                    if !non_zero.contains(&&u) {
                        return None;
                    }

                    return Some((v, d));
                })
                .collect::<HashMap<_, _>>();

            let flow = edges.get(&u).unwrap().0;
            return Some((u, (flow, dist)));
        })
        .collect::<HashMap<_, _>>();

    let mut result: Vec<Vec<usize>> = vec![vec![0; non_zero.len()]; non_zero.len()];
    let mut flows: Vec<usize> = vec![0; non_zero.len()];
    for i in 0..result.len() {
        let (flow, ns) = dists.get(&non_zero[i]).unwrap();
        flows[i] = *flow;
        for j in 0..result[i].len() {
            result[i][j] = *ns.get(&non_zero[j]).unwrap();
        }
    }

    return Input { mapping: non_zero, dists: result, flows, start: 0 };
}

fn parse_input(input: impl AsRef<str>) -> Input {
    let pairs = aoc::parsing::lines_to_vec::<Pair>(input.as_ref())
        .expect("to have correct aoc input")
        .into_iter()
        .map(|p| (p.name, p.node))
        .collect::<HashMap<String, (usize, Vec<String>)>>();

    return shortest_paths(&pairs);
}

fn compute_dp(input: &Input) -> Vec<Vec<Vec<i64>>> {
    let location_size = input.mapping.len();
    let bitset_size = 1 << location_size;
    let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![i64::MIN; bitset_size]; location_size]; 31];

    for k in 0..location_size {
        let d = input.dists[input.start][k];
        dp[d + 1][k][1 << k] = 0;
    }

    for i in 1..dp.len() {
        for j in 0..bitset_size {
            for k in 0..location_size {
                let flow = (0..location_size).filter_map(|i| {
                    if (1 << i) & j == 0 {
                        return None;
                    } else {
                        return Some(input.flows[i]);
                    }
                }).sum::<usize>() as i64;

                let hold = dp[i - 1][k][j] + flow;
                if hold > dp[i][k][j] {
                    dp[i][k][j] = hold;
                }

                if (1 << k) & j == 0 {
                    continue;
                }

                for l in 0..location_size {
                    if (1 << l) & j != 0 {
                        continue;
                    }

                    let d = input.dists[k][l];

                    if i + d + 1 >= dp.len() {
                        continue;
                    }

                    let walk_flow = dp[i][k][j] + flow * (d as i64 + 1);
                    if walk_flow > dp[i + d + 1][l][j | (1 << l)] {
                        dp[i + d + 1][l][j | (1 << l)] = walk_flow;
                    }
                }
            }
        }
    }

    return dp;
}

fn part1(dp: &Vec<Vec<Vec<i64>>>) -> String {
    let mut ans = 0;

    for k in 0..dp[0].len() {
        for j in 0..dp[0][k].len() {
            ans = ans.max(dp[30][k][j]);
        }
    }

    return ans.to_string();
}

fn part2(dp: &Vec<Vec<Vec<i64>>>) -> String {
    let mut ans = 0;

    for i in 0..dp[0][0].len() {
        for j in 0..dp[0][0].len() {
            if i & j != 0 {
                continue;
            }

            let mut a = i64::MIN;
            let mut b = i64::MIN;

            for k in 0..dp[0].len() {
                a = a.max(dp[26][k][i]);
                b = b.max(dp[26][k][j]);
            }

            if let Some(num) = a.checked_add(b) {
                ans = ans.max(num);
            }
        }
    }

    return ans.to_string();
}

fn main() {
    let input = parse_input(aoc::input::read_from_stdin());

    let dp = compute_dp(&input);

    println!("Part 1: {}", part1(&dp));
    println!("Part 2: {}", part2(&dp));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example1() {
        let input = super::parse_input("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II");

        assert_eq!(super::part1(&super::compute_dp(&input)), "1651");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II");


        assert_eq!(super::part2(&super::compute_dp(&input)), "1707");
    }
}
