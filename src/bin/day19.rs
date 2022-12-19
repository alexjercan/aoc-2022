use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Mineral {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type Robot = BTreeMap<Mineral, usize>;

#[derive(Debug)]
struct Blueprint {
    costs: BTreeMap<Mineral, Robot>,
}

impl FromStr for Blueprint {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s
            .split_once(": ")
            .ok_or(Self::Err::ParseError("expect blueprint name".to_string()))?;
        let robots: Vec<&str> = s.split(". ").collect();

        let ore: Vec<&str> = robots[0].split(" ").collect();
        let ore = BTreeMap::from([(
            Mineral::Ore,
            ore[4]
                .parse::<usize>()
                .map_err(|e| Self::Err::ParseError(e.to_string()))?,
        )]);
        let clay: Vec<&str> = robots[1].split(" ").collect();
        let clay = BTreeMap::from([(
            Mineral::Ore,
            clay[4]
                .parse::<usize>()
                .map_err(|e| Self::Err::ParseError(e.to_string()))?,
        )]);
        let obsidian: Vec<&str> = robots[2].split(" ").collect();
        let obsidian = BTreeMap::from([
            (
                Mineral::Ore,
                obsidian[4]
                    .parse::<usize>()
                    .map_err(|e| Self::Err::ParseError(e.to_string()))?,
            ),
            (
                Mineral::Clay,
                obsidian[7]
                    .parse::<usize>()
                    .map_err(|e| Self::Err::ParseError(e.to_string()))?,
            ),
        ]);
        let geode: Vec<&str> = robots[3].split(" ").collect();
        let geode = BTreeMap::from([
            (
                Mineral::Ore,
                geode[4]
                    .parse::<usize>()
                    .map_err(|e| Self::Err::ParseError(e.to_string()))?,
            ),
            (
                Mineral::Obsidian,
                geode[7]
                    .parse::<usize>()
                    .map_err(|e| Self::Err::ParseError(e.to_string()))?,
            ),
        ]);

        let costs = BTreeMap::from([
            (Mineral::Ore, ore),
            (Mineral::Clay, clay),
            (Mineral::Obsidian, obsidian),
            (Mineral::Geode, geode),
        ]);

        return Ok(Blueprint { costs });
    }
}

fn parse_input(input: impl AsRef<str>) -> Vec<Blueprint> {
    aoc::parsing::lines_to_vec::<Blueprint>(input.as_ref()).expect("correct aoc input")
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Inventory {
    minerals: BTreeMap<Mineral, usize>,
    robots: BTreeMap<Mineral, usize>,
}

impl Inventory {
    fn try_build(self: &Self, r: &Mineral, cost: &Robot) -> Option<Self> {
        let mut minerals = self.minerals.clone();
        let mut robots = self.robots.clone();

        for (k, v) in cost {
            let new_v = minerals.get_mut(k)?;
            *new_v = new_v.checked_sub(*v)?;
        }

        robots.entry(r.clone()).and_modify(|e| *e += 1).or_insert(1);

        return Some(Inventory { minerals, robots });
    }

    fn neighbors(self: &Self, bp: &Blueprint) -> Vec<Self> {
        let mut ns: Vec<Self> = bp
            .costs
            .iter()
            .filter_map(|(r, cost)| self.try_build(r, cost))
            .collect();

        ns.push(self.clone());

        return ns;
    }
}

struct Solution {
    top: BTreeMap<Mineral, usize>,
    memo: BTreeMap<Inventory, usize>,
}

impl Solution {
    fn new(bp: &Blueprint) -> Self {

        let top = BTreeMap::from([
            (Mineral::Ore, *bp.costs.iter().map(|(_, r)| r.get(&Mineral::Ore).unwrap_or(&0)).max().unwrap_or(&0)),
            (Mineral::Clay, *bp.costs.iter().map(|(_, r)| r.get(&Mineral::Clay).unwrap_or(&0)).max().unwrap_or(&0)),
            (Mineral::Obsidian, *bp.costs.iter().map(|(_, r)| r.get(&Mineral::Obsidian).unwrap_or(&0)).max().unwrap_or(&0)),
            (Mineral::Geode, usize::MAX),
        ]);

        return Self { memo: BTreeMap::new(), top };
    }

    fn neighbors(self: &Self, inv: &Inventory, bp: &Blueprint) -> Vec<Inventory> {
        if let Some(n) = inv.try_build(&Mineral::Geode, bp.costs.get(&Mineral::Geode).unwrap()) {
            return vec![n];
        }

        return inv.neighbors(bp).into_iter().filter_map(|n| {
            if self.top.iter().all(|(k, v)| n.robots.get(k).unwrap_or(&0) <= v) {
                return Some(n);
            } else {
                return None;
            }
        }).collect();
    }

    fn step(self: &Self, inv: &Inventory, ns: &mut Vec<Inventory>) {
        for n in ns {
            for (k, v) in &inv.robots {
                n.minerals.entry(k.clone()).and_modify(|e| *e += v).or_insert(v.clone());
            }
        }
    }

    fn quality(self: &mut Self, inv: &Inventory, bp: &Blueprint, steps: usize) -> usize {
        if steps <= 0 {
            return *inv.minerals.get(&Mineral::Geode).unwrap_or(&0);
        }

        let mut ns = self.neighbors(inv, bp);

        self.step(inv, &mut ns);

        return ns
            .into_iter()
            .map(|n| self.quality(&n, bp, steps - 1))
            .max()
            .unwrap_or(0);
    }
}

fn part1(input: &Vec<Blueprint>) -> String {
    let init = Inventory {
        minerals: BTreeMap::new(),
        robots: BTreeMap::from([(Mineral::Ore, 1)]),
    };

    let input = &input[0..1];

    return (0..input.len())
        .fold(0, |acc, i| {
            let mut sol = Solution::new(&input[i]);
            return acc + sol.quality(&init, &input[i], 26) * (i + 1)
        })
        .to_string();
}

fn part2(input: &Vec<Blueprint>) -> String {
    "".to_owned()
}

fn main() {
    let input = parse_input(aoc::input::read_from_stdin());

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    // #[test]
    fn part1_example1() {
        let input = super::parse_input("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");

        assert_eq!(super::part1(&input), "33");
    }

    // #[test]
    fn part2_example1() {
        let input = super::parse_input("");

        assert_eq!(super::part2(&input), "");
    }
}
