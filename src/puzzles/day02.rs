use std::fmt::Display;

use anyhow::Error;
use regex::Regex;

#[derive(Debug)]
struct MyNumber {
    x: i64,
    y: i64,
}

impl Display for MyNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl MyNumber {
    pub const fn mult(&self, other: &Self) -> Self {
        Self {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x,
        }
    }

    pub const fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub const fn div(&self, other: &Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

pub fn solve1(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let re = Regex::new(r"A=\[(\d+),(\d+)\]")?;
    let caps = re
        .captures(data)
        .ok_or_else(|| anyhow::anyhow!("Invalid format for number"))?;
    let start = MyNumber {
        x: caps[1].parse::<i64>()?,
        y: caps[2].parse::<i64>()?,
    };

    let mut res = MyNumber { x: 0, y: 0 };

    for _ in 0..3 {
        res = res.mult(&res);
        res = res.div(&MyNumber { x: 10, y: 10 });
        res = res.add(&start);
    }

    Ok(res.to_string())
}

const fn result_exceeds(num: &MyNumber) -> bool {
    num.x > 1_000_000 || num.x < -1_000_000 || num.y > 1_000_000 || num.y < -1_000_000
}

fn should_engrave(num: &MyNumber) -> bool {
    let mut res = MyNumber { x: 0, y: 0 };

    for _ in 0..100 {
        res = res.mult(&res);
        res = res.div(&MyNumber {
            x: 100_000,
            y: 100_000,
        });
        res = res.add(num);

        if result_exceeds(&res) {
            return false;
        }
    }

    true
}

pub fn solve2(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let re = Regex::new(r"A=\[(-?\d+),(-?\d+)\]")?;
    let caps = re
        .captures(data)
        .ok_or_else(|| anyhow::anyhow!("invalid format for number"))?;

    let start = MyNumber {
        x: caps[1].parse::<i64>()?,
        y: caps[2].parse::<i64>()?,
    };

    let mut engraving = 0;

    for i in 0..101 {
        for j in 0..101 {
            let point = start.add(&MyNumber {
                x: 10 * i,
                y: 10 * j,
            });
            if should_engrave(&point) {
                engraving += 1;
            }
        }
    }

    Ok(engraving.to_string())
}

pub fn solve3(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let re = Regex::new(r"A=\[(-?\d+),(-?\d+)\]")?;
    let caps = re
        .captures(data)
        .ok_or_else(|| anyhow::anyhow!("invalid format for number"))?;

    let start = MyNumber {
        x: caps[1].parse::<i64>()?,
        y: caps[2].parse::<i64>()?,
    };

    let mut engraving = 0;

    for i in 0..1001 {
        for j in 0..1001 {
            let point = start.add(&MyNumber { x: i, y: j });
            if should_engrave(&point) {
                engraving += 1;
            }
        }
    }

    Ok(engraving.to_string())
}
