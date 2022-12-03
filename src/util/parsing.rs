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
