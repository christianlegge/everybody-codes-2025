use anyhow::Error;
use everybody_codes_2025::util::parse_lines;

pub fn solve1(data: &str) -> Result<String, Error> {
    println!("Text input: {}", data);
    match parse_lines::<i32>(data) {
        Ok(inputs) => Ok((2025 * inputs[0] / inputs[inputs.len() - 1]).to_string()),
        Err(err) => Err(err),
    }
}

pub fn solve2(data: &str) -> Result<String, Error> {
    println!("Text input: {}", data);
    match parse_lines::<i32>(data) {
        Ok(inputs) => Ok(((10_000_000_000_000_f64 * inputs[inputs.len() - 1] as f64
            / inputs[0] as f64)
            .ceil())
        .to_string()),
        Err(err) => Err(err),
    }
}

#[derive(Debug)]
enum Gear {
    Single(u64),
    Coupled(u64, u64),
}

pub fn solve3(data: &str) -> Result<String, Error> {
    let lines = data.split_whitespace();
    let mut gears: Vec<Gear> = Vec::new();
    for line in lines {
        let parts = line.split("|").collect::<Vec<&str>>();
        gears.push(match parts.len() {
            1 => Gear::Single(parts[0].parse::<u64>().unwrap()),
            2 => Gear::Coupled(
                parts[0].parse::<u64>().unwrap(),
                parts[1].parse::<u64>().unwrap(),
            ),
            _ => panic!("something happened {} {:#?}", line, parts),
        })
    }

    if let Gear::Single(first_gear) = gears[0] {
        Ok(gears
            .iter()
            .fold((first_gear, 100f64), |a: (u64, f64), v| {
                println!("{} turns; gear: {:#?}", a.1, v);
                match v {
                    Gear::Single(x) => (*x, a.1 * a.0 as f64 / *x as f64),
                    Gear::Coupled(x, y) => (*y, a.1 * a.0 as f64 / *x as f64),
                }
            })
            .1
            .floor()
            .to_string())
    } else {
        Err(anyhow::anyhow!("error parsing gears"))
    }
}
