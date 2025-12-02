use anyhow::Error;
use everybody_codes_2025::util::parse_csv;
use itertools::Itertools;

pub fn solve1(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let nums = parse_csv::<i32>(data)?;
    let mut total = 0;
    for (&x, &y) in nums.iter().tuple_windows() {
        println!("{x} {y}");
        total += i32::from((x - y).abs() == 16);
    }
    // let passes = nums
    //     .iter()
    //     .tuple_windows()
    //     .filter(|(&x, &y)| (x - y).abs() == 16)
    //     .try_len()
    //     .unwrap();
    Ok(total.to_string())
}

pub fn solve2(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    // let data = "1,5,2,6,8,4,1,7,3,5,7,8,2";
    let pegs = 256;
    let nums = parse_csv::<i32>(data)?;
    let mut crosses = 0;
    for i in 0..nums.len() {
        if i < 2 {
            println!("step {} invalid, skipping", i + 1);
            continue;
        }
        let cur = &nums[i];
        let last = &nums[i - 1];
        let split = (last - cur).rem_euclid(pegs);
        let mut local_crosses = 0;
        for (&x, &y) in nums[0..i - 1].iter().tuple_windows() {
            if x == *cur || y == *cur || x == *last || y == *last {
                continue;
            }
            if ((x - cur).rem_euclid(pegs) > split) != ((y - cur).rem_euclid(pegs) > split) {
                println!("counting string {x}-{y}");
                local_crosses += 1;
            }
        }
        crosses += local_crosses;
        dbg!(local_crosses);
    }
    Ok(crosses.to_string())
}

pub fn solve3(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    Ok("Unimplemented".to_string())
}
