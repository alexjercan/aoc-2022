use std::fmt::Debug;
use std::str::FromStr;

pub fn lines_to_vec<I, R>(input: I) -> Result<Vec<R>, R::Err>
where
    I: AsRef<str>,
    R: FromStr,
    <R as FromStr>::Err: Debug,
{
    return input
        .as_ref()
        .lines()
        .map(|line| line.parse::<R>())
        .collect::<Result<_, _>>();
}
