use std::str::FromStr;

use anyhow::Error;
use everybody_codes_2025::util::parse_lines;
use itertools::Itertools;

#[derive(Debug)]
struct Scale {
    id: i32,
    code: String,
}

fn find_children(scales: &[Scale]) -> i32 {
    let mut sum = 0;
    for i in scales {
        if let Some((p1, p2)) = find_parents(i, scales) {
            sum += find_similarity(&i.code, &p1.code, &p2.code);
        }
    }
    sum
}

fn find_parents<'a>(child: &Scale, scales: &'a [Scale]) -> Option<(&'a Scale, &'a Scale)> {
    for i in scales {
        if i.id == child.id {
            continue;
        }
        let skip2 = i.id;
        for j in scales {
            if j.id == child.id || j.id == skip2 {
                continue;
            }
            if find_child(&child.code, &i.code, &j.code) {
                return Some((i, j));
            }
        }
    }
    None
}

impl FromStr for Scale {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        Ok(Self {
            id: parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("unable to find id: {s}"))?
                .parse()?,
            code: parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("unable to find code: {s}"))?
                .to_string(),
        })
    }
}

fn find_child(child: &str, parent1: &str, parent2: &str) -> bool {
    for ((c, p1), p2) in child
        .chars()
        .zip_eq(parent1.chars())
        .zip_eq(parent2.chars())
    {
        if c != p1 && c != p2 {
            return false;
        }
    }
    true
}

fn find_similarity(child: &str, parent1: &str, parent2: &str) -> i32 {
    let mut similarity = 1;
    for parent in &[parent1, parent2] {
        let mut sum = 0;
        for (s, o) in child.chars().zip_eq(parent.chars()) {
            sum += i32::from(s == o);
        }
        similarity *= sum;
    }
    similarity
}

pub fn solve1(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let scales: Vec<Scale> = parse_lines(data)?;

    match &scales[..] {
        [one, two, three] => {
            if find_child(&one.code, &two.code, &three.code) {
                Ok(find_similarity(&one.code, &two.code, &three.code).to_string())
            } else if find_child(&two.code, &one.code, &three.code) {
                Ok(find_similarity(&two.code, &one.code, &three.code).to_string())
            } else if find_child(&three.code, &one.code, &two.code) {
                Ok(find_similarity(&three.code, &one.code, &two.code).to_string())
            } else {
                Err(anyhow::anyhow!("child not found"))
            }
        }
        _ => Err(anyhow::anyhow!("wrong number of scales")),
    }
}

pub fn solve2(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    let scales: Vec<Scale> = data.lines().map(Scale::from_str).try_collect()?;
    let s = find_children(&scales);
    Ok(s.to_string())
}

pub fn solve3(data: &str) -> Result<String, Error> {
    println!("Text input: {data}");
    Ok("Unimplemented".to_string())
}
