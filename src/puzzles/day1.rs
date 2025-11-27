pub fn solve(data: String) {
    println!("Text input: {}", data);
    let mut data = data.split("\n");
    let names = data.next().unwrap().split(",").collect::<Vec<&str>>();
    data.next();
    let moves = data.next().unwrap().split(",");
    let mut idx = 0;
    for m in moves {
        let sign: i32 = match &m.chars().nth(0) {
            Some('L') => -1,
            Some('R') => 1,
            _ => panic!("error parsing string {}", m),
        };
        let dist = match (&m[1..]).parse::<i32>() {
            Ok(num) => num,
            Err(e) => panic!("error parsing string {}: {}", m, e),
        };
        let off = dist * sign;
        let new_idx = (idx + off).clamp(0, (names.len() - 1).try_into().unwrap());
        idx = new_idx;

        println!("{:#?} {} {}", sign, dist, idx);
    }
    println!("name: {}", names[idx as usize])
}

pub fn solve2(data: String) {
    println!("Text input: {}", data);
    let mut data = data.split("\n");
    let names = data.next().unwrap().split(",").collect::<Vec<&str>>();
    data.next();
    let moves = data.next().unwrap().split(",");
    let mut idx = 0;
    for m in moves {
        let sign: i32 = match &m.chars().nth(0) {
            Some('L') => -1,
            Some('R') => 1,
            _ => panic!("error parsing string {}", m),
        };
        let dist = match (&m[1..]).parse::<i32>() {
            Ok(num) => num,
            Err(e) => panic!("error parsing string {}: {}", m, e),
        };
        let off = dist * sign;
        let new_idx = idx + off;
        idx = new_idx;

        println!("{:#?} {} {}", sign, dist, idx);
    }
    println!("name: {}", names[(idx % names.len() as i32) as usize])
}

pub fn solve3(data: String) {
    println!("Text input: {}", data);
    let mut data = data.split("\n");
    let mut names = data.next().unwrap().split(",").collect::<Vec<&str>>();
    data.next();
    let moves = data.next().unwrap().split(",");
    for m in moves {
        let sign: i32 = match &m.chars().nth(0) {
            Some('L') => -1,
            Some('R') => 1,
            _ => panic!("error parsing string {}", m),
        };
        let dist = match (&m[1..]).parse::<i32>() {
            Ok(num) => num,
            Err(e) => panic!("error parsing string {}: {}", m, e),
        };
        let off = dist * sign;
        let swap_idx = off.rem_euclid(names.len() as i32);
        println!("{:#?} {} {} {}", sign, dist, off, swap_idx);
        names.swap(0, swap_idx as usize);
    }
    println!("name: {}", names[0])
}
