use std::str::FromStr;

type Recipe = [usize; 4];

#[derive(Debug)]
struct Blueprint {
    recipes: [Recipe; 4],
}

impl FromStr for Blueprint {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace();

        let ore_recipe = [iter.nth(6).unwrap().parse::<usize>().unwrap(), 0, 0, 0];
        let clay_recipe = [iter.nth(5).unwrap().parse::<usize>().unwrap(), 0, 0, 0];
        let obsidian_recipe = [
            iter.nth(5).unwrap().parse::<usize>().unwrap(),
            iter.nth(2).unwrap().parse::<usize>().unwrap(),
            0,
            0,
        ];
        let geode_recipe = [
            iter.nth(5).unwrap().parse::<usize>().unwrap(),
            0,
            iter.nth(2).unwrap().parse::<usize>().unwrap(),
            0,
        ];
        return Ok(Blueprint {
            recipes: [ore_recipe, clay_recipe, obsidian_recipe, geode_recipe],
        });
    }
}

fn parse_input(input: impl AsRef<str>) -> Vec<Blueprint> {
    aoc::parsing::lines_to_vec::<Blueprint>(input.as_ref()).expect("correct aoc input")
}

#[derive(Debug)]
struct State {
    ores: [usize; 4],
    robots: [usize; 4],
    steps: usize,
}

struct Solution {
    best: usize,
}

impl Solution {
    fn new() -> Self {
        Self { best: 0 }
    }

    fn recipe_delay(self: &Self, state: &State, recipe: &Recipe) -> usize {
        return (0..4)
            .filter_map(|ore| {
                if recipe[ore] == 0 {
                    return None;
                } else if recipe[ore] < state.ores[ore] {
                    return Some(0);
                } else if state.robots[ore] <= 0 {
                    return Some(usize::MAX);
                } else {
                    return Some(
                        (recipe[ore] - state.ores[ore] + state.robots[ore] - 1) / state.robots[ore],
                    );
                }
            })
            .max()
            .unwrap_or(0);
    }

    fn winnable(self: &Self, state: &State, max_steps: usize) -> bool {
        let left_steps = max_steps - state.steps;
        return (left_steps - 1) * left_steps / 2 + state.ores[3] + left_steps * state.robots[3]
            >= self.best;
    }

    fn neighbors(
        self: &Self,
        bp: &Blueprint,
        state: &State,
        max_steps: usize,
        robots_cap: &[usize; 4],
    ) -> Vec<State> {
        return (0..4)
            .filter_map(|i| {
                if state.robots[i] == robots_cap[i] {
                    return None;
                }

                let recipe = bp.recipes[i];

                let delay_steps = self.recipe_delay(state, &recipe);
                let new_steps = (state.steps + 1).checked_add(delay_steps)?;

                if new_steps >= max_steps {
                    return None;
                }

                let mut new_ores = [0; 4];
                let mut new_robots = [0; 4];
                for ore in 0..4 {
                    new_ores[ore] =
                        state.ores[ore] + state.robots[ore] * (delay_steps + 1) - recipe[ore];
                    new_robots[ore] = state.robots[ore] + usize::from(ore == i);
                }

                let new_state = State {
                    steps: new_steps,
                    ores: new_ores,
                    robots: new_robots,
                };

                if !self.winnable(&new_state, max_steps) {
                    return None;
                }

                return Some(new_state);
            })
            .collect();
    }

    fn quality(
        self: &mut Self,
        bp: &Blueprint,
        state: &State,
        max_steps: usize,
        robots_cap: &[usize; 4],
    ) -> usize {
        let ns: Vec<State> = self.neighbors(bp, state, max_steps, robots_cap);

        let value = if ns.is_empty() {
            state.ores[3] + state.robots[3] * (max_steps - state.steps)
        } else {
            ns.iter()
                .map(|n| self.quality(bp, n, max_steps, robots_cap))
                .max()
                .unwrap_or(0)
        };

        self.best = self.best.max(value);
        return value;
    }
}

fn solution(bp: &Blueprint, max_steps: usize) -> usize {
    let mut robots_cap = [usize::MAX; 4];
    for i in 0..3 {
        robots_cap[i] = bp.recipes.iter().map(|r| r[i]).max().unwrap();
    }

    let mut sol = Solution::new();

    return sol.quality(
        bp,
        &State {
            steps: 0,
            robots: [1, 0, 0, 0],
            ores: [0, 0, 0, 0],
        },
        max_steps,
        &robots_cap,
    );
}

fn part1(input: &Vec<Blueprint>) -> String {
    (0..input.len())
        .fold(0, |acc, i| acc + solution(&input[i], 24) * (i + 1))
        .to_string()
}

fn part2(input: &Vec<Blueprint>) -> String {
    input
        .iter()
        .take(3)
        .fold(1, |acc, bp| acc * solution(&bp, 32))
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
        let input = super::parse_input("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");

        assert_eq!(super::part1(&input), "33");
    }

    #[test]
    fn part2_example1() {
        let input = super::parse_input("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");

        assert_eq!(super::part2(&input), "3472");
    }
}
