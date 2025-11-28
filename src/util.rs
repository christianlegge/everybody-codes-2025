pub fn comma_split_numbers(s: String) -> Vec<i32> {
    s.split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

pub fn line_split_numbers(s: String) -> Vec<i64> {
    s.split("\n")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}
