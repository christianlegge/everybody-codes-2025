use anyhow::Error;
use hashbrown::{HashMap, HashSet};

pub fn solve1(data: &str) -> Result<String, Error> {
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
    if let Some(s) = sum {
        Ok(s.to_string())
    } else {
        Err(anyhow::anyhow!("unable to reduce sum"))
    }
}

pub fn solve2(data: &str) -> Result<String, Error> {
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
    if let Some(s) = sum {
        Ok(s.to_string())
    } else {
        Err(anyhow::anyhow!("unable to reduce sum"))
    }
}

pub fn solve3(data: &str) -> Result<String, Error> {
    println!("Text input: {}", data);
    let crates = data
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut crate_set: HashMap<u32, u32> = HashMap::new();
    let mut max = 1;
    for ele in crates {
        if crate_set.contains_key(&ele) {
            let prev = crate_set.get(&ele).unwrap().to_owned();
            crate_set.insert(ele, prev + 1);
            if prev + 1 > max {
                max = prev + 1;
            }
        } else {
            crate_set.insert(ele, 1);
        }
    }
    Ok(max.to_string())
}
