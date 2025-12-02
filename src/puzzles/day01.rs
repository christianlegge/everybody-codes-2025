use anyhow::Error;

pub fn solve1(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let mut lines = data.lines();
    let names = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("unable to find first line"))?
        .split(',')
        .collect::<Vec<&str>>();
    lines.next();
    let moves = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("unable to find moves line"))?
        .split(',');
    let mut idx = 0;
    for m in moves {
        let sign: i32 = match &m.chars().next() {
            Some('L') => -1,
            Some('R') => 1,
            Some(c) => return Err(anyhow::anyhow!("Invalid move character {c}")),
            None => return Err(anyhow::anyhow!("Empty move found")),
        };
        let dist = match (m[1..]).parse::<i32>() {
            Ok(num) => num,
            Err(e) => return Err(anyhow::anyhow!("error parsing string {m}: {e}")),
        };
        let off = dist * sign;
        let new_idx = (idx + off).clamp(0, (names.len() - 1).try_into()?);
        idx = new_idx;

        println!("{sign:#?} {dist} {idx}");
    }
    Ok(names[usize::try_from(idx)?].to_string())
}

pub fn solve2(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let mut data = data.lines();
    let names = data
        .next()
        .ok_or_else(|| anyhow::anyhow!("unable to find first line"))?
        .split(',')
        .collect::<Vec<&str>>();
    data.next();
    let moves = data
        .next()
        .ok_or_else(|| anyhow::anyhow!("unable to find moves line"))?
        .split(',');
    let mut idx = 0;
    for m in moves {
        let sign: i32 = match &m.chars().next() {
            Some('L') => -1,
            Some('R') => 1,
            Some(c) => return Err(anyhow::anyhow!("Invalid move character {c}")),
            None => return Err(anyhow::anyhow!("Empty move found")),
        };
        let dist = match (m[1..]).parse::<i32>() {
            Ok(num) => num,
            Err(e) => return Err(anyhow::anyhow!("error parsing string {m}: {e}")),
        };
        let off = dist * sign;
        let new_idx = idx + off;
        idx = new_idx;

        println!("{sign:#?} {dist} {idx}");
    }
    Ok(names[usize::try_from(idx % i32::try_from(names.len())?)?].to_string())
}

pub fn solve3(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let mut data = data.lines();
    let mut names = data
        .next()
        .ok_or_else(|| anyhow::anyhow!("unable to find first line"))?
        .split(',')
        .collect::<Vec<&str>>();
    data.next();
    let moves = data
        .next()
        .ok_or_else(|| anyhow::anyhow!("unable to find moves line"))?
        .split(',');
    for m in moves {
        let sign: i32 = match &m.chars().next() {
            Some('L') => -1,
            Some('R') => 1,
            Some(c) => return Err(anyhow::anyhow!("Invalid move character {c}")),
            None => return Err(anyhow::anyhow!("Empty move found")),
        };
        let dist = match (m[1..]).parse::<i32>() {
            Ok(num) => num,
            Err(e) => return Err(anyhow::anyhow!("error parsing string {m}: {e}")),
        };
        let off = dist * sign;
        let swap_idx = off.rem_euclid(names.len().try_into()?);
        println!("{sign:#?} {dist} {off} {swap_idx}",);
        names.swap(0, swap_idx.try_into()?);
    }
    Ok(names[0].to_string())
}
