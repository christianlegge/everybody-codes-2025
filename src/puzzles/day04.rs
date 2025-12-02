use anyhow::Error;
use everybody_codes_2025::util::parse_lines;

pub fn solve1(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    match parse_lines::<i32>(data) {
        Ok(inputs) => Ok((2025 * inputs[0] / inputs[inputs.len() - 1]).to_string()),
        Err(err) => Err(err),
    }
}

pub fn solve2(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    match parse_lines::<i32>(data) {
        Ok(inputs) => Ok(
            ((10_000_000_000_000_f64 * f64::from(inputs[inputs.len() - 1]) / f64::from(inputs[0]))
                .ceil())
            .to_string(),
        ),
        Err(err) => Err(err),
    }
}

#[derive(Debug)]
enum Gear {
    Single(i32),
    Coupled(i32, i32),
}

pub fn solve3(data: &str) -> Result<String, Error> {
    let lines = data.split_whitespace();
    let mut gears: Vec<Gear> = Vec::new();
    for line in lines {
        let parts = line.split('|').collect::<Vec<&str>>();
        gears.push(match parts.len() {
            1 => Gear::Single(parts[0].parse::<i32>()?),
            2 => Gear::Coupled(parts[0].parse::<i32>()?, parts[1].parse::<i32>()?),
            _ => return Err(anyhow::anyhow!("something happened {line} {parts:#?}")),
        });
    }

    if let Gear::Single(first_gear) = gears[0] {
        Ok(gears
            .iter()
            .fold((first_gear, 100f64), |a: (i32, f64), v| {
                println!("{} turns; gear: {:#?}", a.1, v);
                match v {
                    Gear::Single(x) => (*x, a.1 * f64::from(a.0) / f64::from(*x)),
                    Gear::Coupled(x, y) => (*y, a.1 * f64::from(a.0) / f64::from(*x)),
                }
            })
            .1
            .floor()
            .to_string())
    } else {
        Err(anyhow::anyhow!("error parsing gears"))
    }
}
