mod puzzles;

use seq_macro::seq;

fn get_puzzle_arg() -> Option<String> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        Some(args[1].to_string())
    } else {
        None
    }
}

#[allow(clippy::cognitive_complexity, clippy::unnecessary_semicolon)]
fn solve_puzzle(arg: &str) {
    seq!(N in 01..=20 {
        match arg {
        #(
        stringify!(q~N) => {
            seq!(I in 1..=3 {
                if let Ok(data) = std::fs::read_to_string(format!("data/everybody_codes_e2025_q{:02}_p{}.txt", N, I)) {
                    match crate::puzzles::day~N::solve~I(&data) {
                        Ok(s) => {
                            println!("Part {} solution: {s}", I);
                        }
                        Err(e) => println!("Error solving puzzle: {e}")
                    }
                }
                else {
                    println!(stringify!(Unable to find input ~I));
                }
            });
        }
        )*
            _ => panic!("Invalid argument {arg}")
        }
    })
}

fn main() {
    if let Some(arg) = get_puzzle_arg() {
        solve_puzzle(&arg);
    } else {
        println!("No arguments provided");
    }
}
