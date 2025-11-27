use hashbrown::HashSet;
use regex::Regex;

#[derive(Debug)]
struct MyNumber {
    x: i64,
    y: i64,
}

impl MyNumber {
    pub fn mult(&self, other: &Self) -> Self {
        MyNumber {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        MyNumber {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn div(&self, other: &Self) -> Self {
        MyNumber {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

pub fn solve(data: String) {
    println!("Text input: {}", data);
    let crates = data
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut crate_set = HashSet::new();
    for ele in crates {
        crate_set.insert(ele);
    }
    let sum = crate_set.into_iter().reduce(|a, v| a + v);
    println!("sum: {:#?}", sum);
}

fn result_exceeds(num: &MyNumber) -> bool {
    num.x > 1000000 || num.x < -1000000 || num.y > 1000000 || num.y < -1000000
}

fn should_engrave(num: &MyNumber) -> bool {
    let mut res = MyNumber { x: 0, y: 0 };

    for _ in 0..100 {
        res = res.mult(&res);
        res = res.div(&MyNumber {
            x: 100000,
            y: 100000,
        });
        res = res.add(num);

        if result_exceeds(&res) {
            return false;
        }
    }

    true
}

pub fn solve2(data: String) {
    println!("Text input: {}", data);
    let mut crates = data
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut crate_set = HashSet::new();
    crates.sort();
    for ele in crates {
        if crate_set.contains(&ele) {
            continue;
        }
        crate_set.insert(ele);
        if crate_set.len() == 20 {
            break;
        }
    }
    let sum = crate_set.into_iter().reduce(|a, v| a + v);
    println!("sum: {:#?}", sum);
}

pub fn solve3(data: String) {
    println!("Text input: {}", data);
    let re = Regex::new(r"A=\[(-?\d+),(-?\d+)\]").unwrap();
    let Some(caps) = re.captures(&data) else {
        panic!("invalid format for number")
    };

    let start = MyNumber {
        x: (&caps[1]).parse::<i64>().unwrap(),
        y: (&caps[2]).parse::<i64>().unwrap(),
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

    println!("final result: {:#?}", engraving);
}
