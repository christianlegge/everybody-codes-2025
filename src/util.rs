use std::str::FromStr;

use anyhow::{Context, Error};
use itertools::Itertools;

/// # Errors
///
/// Returns an error if any of the strings fail to parse.
pub fn parse_csv<T: FromStr>(s: &str) -> Result<Vec<T>, Error>
where
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    s.split(',')
        .map(|x| x.parse::<T>().with_context(|| format!("cannot parse {s}")))
        .try_collect::<T, Vec<T>, Error>()
}

/// # Errors
///
/// Returns an error if any of the strings fail to parse.
pub fn parse_lines<T: FromStr>(s: &str) -> anyhow::Result<Vec<T>>
where
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    s.lines()
        .map(|x| T::from_str(x).with_context(|| format!("cannot parse {s}")))
        .try_collect::<T, Vec<T>, Error>()
}
