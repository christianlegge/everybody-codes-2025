use std::str::FromStr;

use anyhow::Error;
use everybody_codes_2025::util::parse_csv;
use itertools::Itertools;

#[derive(Debug, Default)]
struct Fishbone {
    segments: Vec<Segment>,
}

#[derive(Debug)]
struct Segment {
    left: Option<i64>,
    spine: i64,
    right: Option<i64>,
}

#[derive(Debug)]
struct Sword {
    identifier: i64,
    fishbone: Fishbone,
}

impl Ord for Sword {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.fishbone.get_quality();
        let b = other.fishbone.get_quality();
        if a == b {
            let a_iter = self.fishbone.segments.iter();
            let b_iter = other.fishbone.segments.iter();
            for (x, y) in a_iter.zip_eq(b_iter) {
                let x_num = x.get_concat_number();
                let y_num = y.get_concat_number();
                if x_num != y_num {
                    return x_num.cmp(&y_num);
                }
            }
            self.identifier.cmp(&other.identifier)
        } else {
            a.cmp(&b)
        }
    }
}

impl PartialOrd for Sword {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Sword {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Eq for Sword {}

impl FromStr for Sword {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(':').collect::<Vec<&str>>();
        let numbers = parse_csv::<i64>(parts[1])?;
        if let Some(id) = parts.first() {
            Ok(Self {
                identifier: id.parse::<i64>()?,
                fishbone: Fishbone::from_numbers(numbers.into_iter()),
            })
        } else {
            Err(anyhow::anyhow!("error constructing sword from {s}"))
        }
    }
}

impl Segment {
    pub fn get_concat_number(&self) -> i64 {
        let mut number = 0;
        if let Some(n) = self.left {
            number *= 10;
            number += &n;
        }
        number *= 10;
        number += &self.spine;
        if let Some(n) = self.right {
            number *= 10;
            number += &n;
        }
        number
    }
}

impl Fishbone {
    pub fn from_numbers<T>(nums: T) -> Self
    where
        T: Iterator<Item = i64>,
    {
        let mut fishbone = Self::default();
        for num in nums {
            fishbone.add_number(num);
        }
        fishbone
    }

    pub fn add_number(&mut self, n: i64) {
        for segment in &mut self.segments {
            if n < segment.spine && segment.left.is_none() {
                segment.left = Some(n);
                return;
            } else if n > segment.spine && segment.right.is_none() {
                segment.right = Some(n);
                return;
            }
        }
        self.segments.push(Segment {
            spine: n,
            left: None,
            right: None,
        });
    }

    pub fn get_quality(&self) -> i64 {
        let mut quality = 0;
        for segment in &self.segments {
            quality *= 10;
            quality += &segment.spine;
        }
        quality
    }
}

pub fn solve1(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let parts = data.split(':').collect::<Vec<&str>>();
    let numbers = parse_csv(parts[1])?;
    let mut fishbone = Fishbone::default();
    for number in numbers {
        fishbone.add_number(number);
    }
    dbg!(&fishbone);
    Ok(fishbone.get_quality().to_string())
}

pub fn solve2(data: &str) -> Result<String, Error> {
    let mut swords: Vec<Sword> = data.lines().map(Sword::from_str).try_collect()?;
    // swords.sort_by(|sword, other| {
    //     sword
    //         .fishbone
    //         .get_quality()
    //         .cmp(&other.fishbone.get_quality())
    // });
    swords.sort();
    dbg!(&swords.first());
    dbg!(&swords.last());
    Ok((swords
        .last()
        .ok_or_else(|| anyhow::anyhow!("unable to get last sword"))?
        .fishbone
        .get_quality()
        - swords
            .first()
            .ok_or_else(|| anyhow::anyhow!("unable to get first sword"))?
            .fishbone
            .get_quality())
    .to_string())
}

pub fn solve3(data: &str) -> Result<String, Error> {
    let mut swords: Vec<Sword> = data.lines().map(Sword::from_str).try_collect()?;
    swords.sort();
    let c = swords
        .iter()
        .rev()
        .enumerate()
        .try_fold(0, |checksum, (i, sword)| {
            Ok((i64::try_from(i)? + 1) * sword.identifier + checksum)
        });
    c.map(|n| n.to_string())
}
