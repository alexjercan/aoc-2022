use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

pub fn lines_to_vec<R>(input: impl AsRef<str>) -> Result<Vec<R>, R::Err>
where
    R: FromStr,
    <R as FromStr>::Err: Debug,
{
    return input
        .as_ref()
        .lines()
        .map(|line| line.parse::<R>())
        .collect::<Result<_, _>>();
}

pub fn char_counts(string: impl AsRef<str>) -> HashMap<char, usize> {
    return string.as_ref().chars().fold(HashMap::new(), |mut acc, c| {
        acc.entry(c).and_modify(|count| *count += 1).or_insert(1);
        return acc;
    });
}
