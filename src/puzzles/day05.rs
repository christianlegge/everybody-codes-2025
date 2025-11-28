use std::str::FromStr;

use aoc2023::util::{comma_split_numbers, line_split_numbers};
use regex::Regex;

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

impl FromStr for Sword {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(":").collect::<Vec<&str>>();
        let numbers = comma_split_numbers::<i64>(parts[1].to_owned());
        if let Some(id) = parts.first() {
            Ok(Sword {
                identifier: id.parse::<i64>().unwrap(),
                fishbone: Fishbone::from_numbers(numbers.into_iter()),
            })
        } else {
            panic!("error constructing sword: {}", s);
        }
    }
}

impl Fishbone {
    pub fn from_numbers<T>(nums: T) -> Self
    where
        T: Iterator<Item = i64>,
    {
        let mut fishbone = Fishbone::default();
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
        let mut quality = String::default();
        for segment in &self.segments {
            quality += &segment.spine.to_string();
        }
        quality.parse().unwrap()
    }
}

pub fn solve(data: String) {
    println!("Text input: {}", data);
    let parts = data.split(":").collect::<Vec<&str>>();
    let numbers = comma_split_numbers(parts[1].to_owned());
    let mut fishbone = Fishbone::default();
    for number in numbers {
        fishbone.add_number(number);
    }
    dbg!(&fishbone);
    println!("quality: {}", fishbone.get_quality());
}

pub fn solve2(data: String) {
    let mut swords = data
        .split_whitespace()
        .map(|s| Sword::from_str(s).unwrap())
        .collect::<Vec<Sword>>();
    swords.sort_by(|sword, other| {
        sword
            .fishbone
            .get_quality()
            .cmp(&other.fishbone.get_quality())
    });
    dbg!(&swords.first());
    dbg!(&swords.last());
    println!(
        "diff: {}",
        swords.first().unwrap().fishbone.get_quality()
            - swords.last().unwrap().fishbone.get_quality()
    );
}

pub fn solve3(data: String) {}
