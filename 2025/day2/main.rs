use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

// part 1
fn is_pattern(n: &str) -> bool {
    let len = n.len();
    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    n[0..half] == n[half..]
}

// part 2
fn is_repeating_pattern_of(n: &str, nlen: usize, pattern: &str, plen: usize) -> bool {
    let mut x = plen; // skip first check since `pattern` is always a prefix of `n`
    while x < nlen {
        if &n[x..x + plen] != pattern {
            return false;
        }
        x += plen;
    }
    return true;
}
fn test_is_repeating_pattern_of() {
    assert_eq!(is_repeating_pattern_of("1212", 4, "12", 2), true);
}
fn is_pattern_2(n: &str) -> bool {
    // pseudocode:
    // for each i that evenly divides len(n):
    //   check if n is exactly n[0..i] * (len(n) / i)
    let len = n.len();
    let half = len / 2 + 1; // stopping at <about> half is fine
    for i in 1..half {
        if len % i == 0 {
            // check
            if is_repeating_pattern_of(n, len, &n[0..i], i) {
                // println!(
                //     "found pattern with n={n} nlen={len} pattern={}, plen={i}",
                //     &n[0..i]
                // );
                return true;
            }
        }
    }
    false
}

fn take_range(reader: &mut BufReader<File>) -> Option<(String, String)> {
    let mut buf: Vec<u8> = vec![];
    let buf_len = reader.read_until(b',', &mut buf).unwrap();
    if buf_len == 0 {
        return None;
    }
    let unparsed = std::str::from_utf8(&buf).unwrap();
    let parts: Vec<&str> = unparsed.split('-').collect();
    let start = parts.get(0).copied().unwrap();
    let end = parts.get(1).copied().unwrap();
    let endpos = if end.chars().nth_back(0) == Some(',') {
        end.len() - 1
    } else {
        end.len()
    };
    Some((start.to_string(), end[..endpos].to_string()))
}

fn main() {
    println!("advent of code!");
    test_is_repeating_pattern_of();

    let args: Vec<String> = env::args().collect();
    let filename = args[1..].first().expect("usage: cargo run -- [input]");
    let file = File::open(filename).unwrap();
    let mut lines_buf = BufReader::new(file);

    let mut invalid_sum = 0;

    while let Some((start, end)) = take_range(&mut lines_buf) {
        println!("pair=({start}, {end})");
        let mut si = start.parse::<i64>().unwrap();
        let ei = end.parse::<i64>().unwrap();
        while si <= ei {
            // if is_pattern(&si.to_string()) {
            if is_pattern_2(&si.to_string()) {
                // println!("found pattern-> {si}");
                invalid_sum += si;
            }
            si += 1;
        }
    }

    println!("invalid_sum={invalid_sum}");
}
