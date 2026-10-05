use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("advent of code!");

    let args: Vec<String> = env::args().collect();
    let filename = args[1..].first().expect("usage: cargo run -- [input]");
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();

    let mut zero_count = 0;
    let mut safe_value = 50;

    for line in lines.map_while(Result::ok) {
        let direction = &line[0..1];
        let ticks = &line[1..];

        let direction = match direction {
            "L" => -1,
            "R" => 1,
            _ => 0,
        };
        let ticks = ticks.parse::<i32>().unwrap();

        let mut c_ticks = ticks;
        while c_ticks > 0 {
            safe_value += direction;

            if safe_value == 0 {
                zero_count += 1;
            }
            if safe_value == -1 {
                safe_value = 99;
            }
            if safe_value == 100 {
                safe_value = 0;
                zero_count += 1;
            }
            c_ticks -= 1;
        }
    }

    println!("zero_count={}", zero_count);
}
