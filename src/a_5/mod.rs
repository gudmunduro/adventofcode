use std::fs;
use std::ops::{Range, RangeInclusive};

pub fn run_a5() {
    let (ranges, ids) = parse_input(&fs::read_to_string("src/a_5/input.txt").unwrap());

    let mut count = 0;
    for id in ids {
        if ranges.iter().any(|r| r.contains(&id)) {
            count += 1;
        }
    }

    println!("Count: {count}");
}

pub fn run_a5_2() {
    let (mut ranges, ids) = parse_input(&fs::read_to_string("src/a_5/input.txt").unwrap());
    ranges.sort_by_key(|r| *r.start());

    let mut count = 0;
    for (i, range) in ranges.iter().enumerate() {
        let range_start = ranges[..i]
            .iter()
            .map(|r| *r.end()+1)
            .max()
            .unwrap_or(0)
            .max(*range.start());

        if range_start <= *range.end() {
            count += (range_start..=(*range.end())).count();
        }
    }

    println!("Count: {count}");
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (ranges_content, ids_content) = input.split_once("\n\n").unwrap();

    let ranges = ranges_content
        .split('\n')
        .map(|v| {
            let (start, end) = v.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();
    let ids = ids_content
        .split('\n').map(|id| id.parse().unwrap())
        .collect();

    (ranges, ids)
}