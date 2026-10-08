use std::{
    collections::BTreeSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

type Interval = (u64, u64);
type IntervalSet = BTreeSet<Interval>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Mode {
    ReadIntervals,
    Check,     // part 1
    Enumerate, // part 2
}

// returns:
// Some(interval) if the two arguments overlap and can be enlarged
// None if the arguments are disjoint and cannot be joined
fn enlarge(existing: &Interval, int: &Interval) -> Option<Interval> {
    let result = (|| {
        if existing.0 <= int.0 && int.1 <= existing.1 {
            // fully contained in the existing interval. do nothing
            return Some(existing.clone());
        }
        // exi 1   4
        // int   3   6
        if existing.0 < int.0 && int.0 <= existing.1 && existing.1 < int.1 {
            // enlarge the existing interval, to the right
            return Some((existing.0, int.1));
        }
        // exi    3   5
        // int  1   4
        if int.0 < existing.0 && existing.0 <= int.1 && int.1 < existing.1 {
            // enlarge the existing interval, to the left
            return Some((int.0, existing.1));
        }
        if int.0 <= existing.0 && existing.0 <= int.1 {
            // fully contains the existing interval, enlarge both ends
            return Some((int.0, int.1));
        }
        None
    })();
    println!(
        "enlarging {:?} with {:?} resulted in {:?}",
        existing, int, result
    );
    result
}

// 1-3  6-8  12-16
// add: 4-5

// has bug (doesn't affect AoC):
//  this function can merge three intervals into one, but not four or more.
fn insert_interval(set: &mut IntervalSet, int: Interval) {
    // find the first interv that is higher than our int
    let top_idx = set.iter().position(|i| i >= &int);
    let top = top_idx.and_then(|top| set.iter().nth(top)).cloned();
    if let Some(top) = top {
        let top_idx = top_idx.unwrap();
        // top found, check overlap with int
        // let mut has_overlap = false;
        let top_joined = enlarge(&top, &int);
        // also check the interval just before top. it might also overlap
        let before = top_idx
            .checked_add_signed(-1)
            .and_then(|top_idx| set.iter().nth(top_idx).cloned());
        let before_joined = before.and_then(|before| enlarge(&before, &int));
        if let (Some(top_joined), Some(before_joined)) = (top_joined, before_joined) {
            let before = before.unwrap();
            // both joins worked: it's all a single happy interval
            let all_joined = (before_joined.0, top_joined.1);
            set.remove(&top);
            set.remove(&before);
            set.insert(all_joined);
        } else if let Some(top_joined) = top_joined {
            // replace only top
            set.remove(&top);
            set.insert(top_joined);
        } else if let Some(before_joined) = before_joined {
            let before = before.unwrap();
            // replace only bottom
            set.remove(&before);
            set.insert(before_joined);
        } else {
            // no join worked. it's a disjoint interval
            set.insert(int);
        }
    } else {
        println!("top not found");
        // we need to check the last interval cause it might need enlargement
        let last = set.last().cloned();
        if let Some(last) = last {
            let enlarged = enlarge(&last, &int);
            if let Some(enlarged) = enlarged {
                // overlap, replace
                set.remove(&last);
                set.insert(enlarged);
            } else {
                // disjoint, add it
                set.insert(int);
            }
        } else {
            // no last interval, this is the first one ever
            set.insert(int);
        }
    }
}

fn main() {
    println!("advent of code!");

    let args: Vec<String> = env::args().collect();
    let filename = args[1..].first().expect("usage: cargo run -- [input]");
    let file = File::open(filename).unwrap();
    let lines_buf = BufReader::new(file);

    let mut intervals: IntervalSet = IntervalSet::new();
    let mut mode = Mode::ReadIntervals;
    let desired_mode = Mode::Enumerate; // part1: Check, part2: Enumerate
    let mut count = 0u64;

    for line in lines_buf.lines().map_while(Result::ok) {
        if line == "" {
            mode = desired_mode;
            println!("Now in mode {mode:?}");
            continue;
        }
        if mode == Mode::ReadIntervals {
            let parts: Vec<&str> = line.split('-').collect();
            let from = parts[0].parse::<u64>().unwrap();
            let to = parts[1].parse::<u64>().unwrap();
            println!("trying to insert {}-{}", from, to);
            insert_interval(&mut intervals, (from, to));
            // print intervals
            for int in &intervals {
                println!("{:4} - {:4}", int.0, int.1);
            }
        } else if mode == Mode::Check {
            let needle = line.parse::<u64>().unwrap();
            let exists = intervals
                .iter()
                .any(|int| int.0 <= needle && needle <= int.1);
            count += if exists { 1 } else { 0 }
        } else if mode == Mode::Enumerate {
            let mut last_end = 0;
            for int in &intervals {
                println!("interv {:?} has {} elements", int, int.1 - int.0 + 1);
                assert!(last_end < int.0, "final intervals are not disjoint");
                last_end = int.1;
                count = count.checked_add(int.1 - int.0 + 1).unwrap();
            }
            break;
        }
    }

    println!("count: {count}");
}
