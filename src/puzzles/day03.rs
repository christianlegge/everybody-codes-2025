use hashbrown::{HashMap, HashSet};
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
    let crates = data
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut crate_set: HashMap<u32, u32> = HashMap::new();
    let mut max = 1;
    for ele in crates {
        if crate_set.contains_key(&ele) {
            let prev = crate_set.get(&ele).unwrap().clone();
            crate_set.insert(ele, prev + 1);
            if prev + 1 > max {
                max = prev + 1;
            }
        } else {
            crate_set.insert(ele, 1);
        }
    }
    println!("max: {:#?}", max);
}
