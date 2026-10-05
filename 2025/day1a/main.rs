use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("advent of code!");

    let file = File::open("input").unwrap();
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

        safe_value += ticks * direction;

        while safe_value < 0 {
            safe_value += 100;
        }
        while safe_value > 99 {
            safe_value -= 100;
        }

        zero_count += if safe_value == 0 { 1 } else { 0 }
    }

    println!("zero_count={}", zero_count);
}
