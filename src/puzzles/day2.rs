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
    let re = Regex::new(r"A=\[(\d+),(\d+)\]").unwrap();
    let Some(caps) = re.captures(&data) else {
        panic!("invalid format for number")
    };

    let start = MyNumber {
        x: (&caps[1]).parse::<i64>().unwrap(),
        y: (&caps[2]).parse::<i64>().unwrap(),
    };

    let mut res = MyNumber { x: 0, y: 0 };

    for _ in 0..3 {
        res = res.mult(&res);
        res = res.div(&MyNumber { x: 10, y: 10 });
        res = res.add(&start);
    }

    println!("final result: {:#?}", res);
}

pub fn solve2(data: String) {
    println!("Text input: {}", data);
    let re = Regex::new(r"A=\[(-?\d+),(-?\d+)\]").unwrap();
    let Some(caps) = re.captures(&data) else {
        panic!("invalid format for number")
    };

    let start = MyNumber {
        x: (&caps[1]).parse::<i64>().unwrap(),
        y: (&caps[2]).parse::<i64>().unwrap(),
    };

    let mut res = MyNumber { x: 0, y: 0 };

    for _ in 0..3 {
        res = res.mult(&res);
        res = res.div(&MyNumber { x: 10, y: 10 });
        res = res.add(&start);
    }

    println!("final result: {:#?}", res);
}

pub fn solve3(data: String) {
    println!("Text input: {}", data);
    let mut data = data.split("\n");
    let mut names = data.next().unwrap().split(",").collect::<Vec<&str>>();
    data.next();
    let moves = data.next().unwrap().split(",");
    for m in moves {
        let sign: i64 = match &m.chars().nth(0) {
            Some('L') => -1,
            Some('R') => 1,
            _ => panic!("error parsing string {}", m),
        };
        let dist = match (&m[1..]).parse::<i64>() {
            Ok(num) => num,
            Err(e) => panic!("error parsing string {}: {}", m, e),
        };
        let off = dist * sign;
        let swap_idx = off.rem_euclid(names.len() as i64);
        println!("{:#?} {} {} {}", sign, dist, off, swap_idx);
        names.swap(0, swap_idx as usize);
    }
    println!("name: {}", names[0])
}
