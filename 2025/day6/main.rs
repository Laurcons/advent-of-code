use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("advent of code!");

    let args: Vec<String> = env::args().collect();
    let filename = args[1..].first().expect("usage: cargo run -- [input]");
    let file = File::open(filename).unwrap();
    let lines_buf = BufReader::new(file);

    let part = 2; // 1 or 2 
    if part == 1 {
        let mut matrix: Vec<Vec<i64>> = vec![];
        let mut operations: Vec<u8> = vec![];

        for line in lines_buf.lines().map_while(Result::ok) {
            println!("line={line}");
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();
            let first_item = parts.first().unwrap().parse::<i64>();
            if first_item.is_err() {
                // we have reached the operations
                operations.extend(parts.iter().map(|p| p.bytes().nth(0).unwrap()));
            } else {
                let parts: Vec<i64> = parts
                    .iter()
                    .map(|str| str.parse::<i64>().unwrap())
                    .collect();
                matrix.push(parts);
            }
        }

        // calculate
        let mut acc;
        let mut sum = 0;
        for col in 0..matrix[0].len() {
            let op = operations[col];
            acc = if op == b'+' { 0 } else { 1 };
            let mut row = 0;
            loop {
                let mrow = matrix.get(row);
                if let Some(mrow) = mrow {
                    let num = mrow[col];
                    if op == b'+' {
                        acc += num;
                    } else {
                        acc *= num;
                    }
                    row += 1;
                } else {
                    break;
                }
            }
            sum += acc;
        }

        println!("sum: {sum}");
    } else if part == 2 {
        let mut lines: Vec<Vec<u8>> = lines_buf
            .lines()
            .map_while(Result::ok)
            .map(|l| l.as_bytes().to_vec())
            .collect();
        let operations: Vec<u8> = lines.pop().unwrap();
        let last_line: &Vec<u8> = lines.last().unwrap();

        for line in &lines {
            println!("{}", String::from_utf8(line.clone()).unwrap());
        }
        println!("{}", String::from_utf8(operations.clone()).unwrap());

        let mut grand_total = 0i64;
        let mut operands: Vec<i64> = vec![];

        for col in (0..lines[0].len()).rev() {
            let least_sig_digit = last_line[col];
            // not a good check: sometimes numbers sit higher in their column
            // if least_sig_digit == b' ' {
            //     continue;
            // }
            let num_bytes: Vec<u8> = lines.iter().map(|l| l[col]).collect();
            let num_string = String::from_utf8(num_bytes).unwrap();
            let num_trimmed = num_string.trim();
            if num_trimmed.len() == 0 {
                continue;
            }
            let num = num_trimmed.parse::<i64>().unwrap();
            operands.push(num);
            println!("identified {num}");
            if operations[col] != b' ' {
                if operations[col] == b'+' {
                    let sum: i64 = operands.iter().sum();
                    println!("calculated sum: {sum}");
                    grand_total += sum;
                }
                if operations[col] == b'*' {
                    let prod: i64 = operands.iter().fold(1, |acc, curr| acc * curr);
                    println!("calculated product: {prod}");
                    grand_total += prod;
                }
                operands.clear();
            }
        }

        println!("grand total: {grand_total}");
    }
}
