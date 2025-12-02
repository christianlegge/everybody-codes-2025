use anyhow::Error;
use everybody_codes_2025::util::parse_csv;
use hashbrown::{HashMap, HashSet};

pub fn solve1(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let crates = parse_csv::<u32>(data)?;
    let mut crate_set = HashSet::new();
    for ele in crates {
        crate_set.insert(ele);
    }
    let sum = crate_set.into_iter().reduce(|a, v| a + v);
    sum.map_or_else(
        || Err(anyhow::anyhow!("unable to reduce sum")),
        |s| Ok(s.to_string()),
    )
}

pub fn solve2(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let mut crates = parse_csv::<u32>(data)?;
    let mut crate_set = HashSet::new();
    crates.sort_unstable();
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
    sum.map_or_else(
        || Err(anyhow::anyhow!("unable to reduce sum")),
        |s| Ok(s.to_string()),
    )
}

pub fn solve3(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let crates = parse_csv::<u32>(data)?;
    let mut crate_set: HashMap<u32, u32> = HashMap::new();
    let mut max = 1;
    for ele in crates {
        if crate_set.contains_key(&ele) {
            let prev = crate_set
                .get(&ele)
                .ok_or_else(|| anyhow::anyhow!("error getting key {ele}"))?
                .to_owned();
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
