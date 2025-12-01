use anyhow::Error;

pub fn solve1(data: &str) -> Result<String, Error> {
    println!("Text input: {}", data);
    let mut data = data.split("\n");
    let names = data.next().unwrap().split(",").collect::<Vec<&str>>();
    data.next();
    let moves = data.next().unwrap().split(",");
    let mut idx = 0;
    for m in moves {
        let sign: i32 = match &m.chars().next() {
            Some('L') => -1,
            Some('R') => 1,
            _ => panic!("error parsing string {}", m),
        };
        let dist = match (m[1..]).parse::<i32>() {
            Ok(num) => num,
            Err(e) => panic!("error parsing string {}: {}", m, e),
        };
        let off = dist * sign;
        let new_idx = (idx + off).clamp(0, (names.len() - 1).try_into().unwrap());
        idx = new_idx;

        println!("{:#?} {} {}", sign, dist, idx);
    }
    Ok(names[idx as usize].to_string())
}

pub fn solve2(data: &str) -> Result<String, Error> {
    println!("Text input: {}", data);
    let mut data = data.split("\n");
    let names = data.next().unwrap().split(",").collect::<Vec<&str>>();
    data.next();
    let moves = data.next().unwrap().split(",");
    let mut idx = 0;
    for m in moves {
        let sign: i32 = match &m.chars().next() {
            Some('L') => -1,
            Some('R') => 1,
            _ => panic!("error parsing string {}", m),
        };
        let dist = match (m[1..]).parse::<i32>() {
            Ok(num) => num,
            Err(e) => panic!("error parsing string {}: {}", m, e),
        };
        let off = dist * sign;
        let new_idx = idx + off;
        idx = new_idx;

        println!("{:#?} {} {}", sign, dist, idx);
    }
    Ok(names[(idx % names.len() as i32) as usize].to_string())
}

pub fn solve3(data: &str) -> Result<String, Error> {
    println!("Text input: {}", data);
    let mut data = data.split("\n");
    let mut names = data.next().unwrap().split(",").collect::<Vec<&str>>();
    data.next();
    let moves = data.next().unwrap().split(",");
    for m in moves {
        let sign: i32 = match &m.chars().next() {
            Some('L') => -1,
            Some('R') => 1,
            _ => panic!("error parsing string {}", m),
        };
        let dist = match (m[1..]).parse::<i32>() {
            Ok(num) => num,
            Err(e) => panic!("error parsing string {}: {}", m, e),
        };
        let off = dist * sign;
        let swap_idx = off.rem_euclid(names.len() as i32);
        println!("{:#?} {} {} {}", sign, dist, off, swap_idx);
        names.swap(0, swap_idx as usize);
    }
    Ok(names[0].to_string())
}
