mod puzzles;

use crate::puzzles::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => {
            println!("No arguments provided");
        }
        _ => {
            let arg = args[1].as_str();
            let data =
                std::fs::read_to_string(format!("data/everybody_codes_e2025_{}_p1.txt", arg));
            let data2 =
                std::fs::read_to_string(format!("data/everybody_codes_e2025_{}_p2.txt", arg));
            let data3 =
                std::fs::read_to_string(format!("data/everybody_codes_e2025_{}_p3.txt", arg));
            match arg {
                "q01" => {
                    println!("============== PART 1 ==============");
                    match data {
                        Ok(data) => day1::solve(data),
                        Err(err) => println!("Error reading data 1: {}", err),
                    };
                    println!("============== PART 2 ==============");
                    match data2 {
                        Ok(data) => day1::solve2(data),
                        Err(err) => println!("Error reading data 2: {}", err),
                    };
                    println!("============== PART 3 ==============");
                    match data3 {
                        Ok(data) => day1::solve3(data),
                        Err(err) => println!("Error reading data 3: {}", err),
                    };
                }
                "q02" => {
                    println!("============== PART 1 ==============");
                    match data {
                        Ok(data) => day2::solve(data),
                        Err(err) => println!("Error reading data 1: {}", err),
                    };
                    println!("============== PART 2 ==============");
                    match data2 {
                        Ok(data) => day2::solve2(data),
                        Err(err) => println!("Error reading data 2: {}", err),
                    };
                    println!("============== PART 3 ==============");
                    match data3 {
                        Ok(data) => day2::solve3(data),
                        Err(err) => println!("Error reading data 3: {}", err),
                    };
                }
                _ => {
                    println!("Invalid argument: {}", args[1]);
                }
            };
        }
    }
}
