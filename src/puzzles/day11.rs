use anyhow::Error;
use itertools::Itertools;
use num::Integer;

struct Ducks {
    columns: Vec<i64>,
    rounds: i64,
}

impl Ducks {
    pub fn phase1(&mut self) -> bool {
        if self.rounds.is_multiple_of(&1000) {
            dbg!(&self.columns);
        }
        let mut changed = false;
        for i in 0..self.columns.len() - 1 {
            let j = i + 1;
            if self.columns[j] < self.columns[i] {
                self.columns[i] -= 1;
                self.columns[j] += 1;
                changed = true;
            }
        }
        self.rounds += i64::from(changed);
        // !changed || self.rounds > 10
        !changed
    }

    pub fn phase2(&mut self) -> bool {
        if self.rounds.is_multiple_of(&1000) {
            dbg!(&self.columns);
        }
        let mut changed = false;
        for i in 0..self.columns.len() - 1 {
            let j = i + 1;
            if self.columns[j] > self.columns[i] {
                self.columns[i] += 1;
                self.columns[j] -= 1;
                changed = true;
            }
        }
        self.rounds += i64::from(changed);
        // !changed || self.rounds > 10
        !changed
    }

    pub fn checksum(&self) -> i64 {
        let mut sum = 0;
        for (i, ducks) in self.columns.iter().enumerate() {
            sum += ducks * i64::try_from(i + 1).expect("error converting index");
        }
        sum
    }
}

pub fn solve1(data: &str) -> Result<String, Error> {
    //     let data = r"9
    // 1
    // 1
    // 4
    // 9
    // 6";
    println!("Text input: {data}");
    let mut ducks = Ducks {
        columns: data.lines().map(str::parse::<i64>).try_collect()?,
        rounds: 0,
    };
    while !ducks.phase1() {}
    while !ducks.phase2() {}
    Ok(ducks.checksum().to_string())
}

pub fn solve2(data: &str) -> Result<String, Error> {
    //     let data = r"9
    // 1
    // 1
    // 4
    // 9
    // 6";
    println!("Text input: {data}");
    let mut ducks = Ducks {
        columns: data.lines().map(str::parse::<i64>).try_collect()?,
        rounds: 0,
    };
    while !ducks.phase1() {}
    while !ducks.phase2() {}
    Ok(ducks.rounds.to_string())
}

pub fn solve3(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");

    // solve2(data)
    Ok("Unimplemented".to_string())
}
