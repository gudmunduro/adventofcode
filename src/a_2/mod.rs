use std::fs;

pub fn run_a2() {
    let ranges = parse_input(&fs::read_to_string("src/a_2/input.txt").unwrap());

    let mut invalid_sum = 0;
    for range in &ranges {
        for num in range.start..=range.end {
            if is_invalid_2(&num.to_string()) {
                invalid_sum += num;
            }
        }
    }

    println!("Invalid: {invalid_sum}");
}

fn is_invalid(id: &str) -> bool {
    if id.len() % 2 != 0 {
        return false;
    }

    let (first, second) = id.split_at(id.len()/2);
    first == second
}

fn is_invalid_2(id: &str) -> bool {
    if id.len() < 2 {
        return false;
    }

    let pattern_len_range = 1..=id.len() / 2;
    for pattern_len in pattern_len_range {
        let repeat_count = id.len() / pattern_len;

        if id[0..pattern_len].repeat(repeat_count) == id {
            return true;
        }
    }

    false
}

struct Range {
    start: usize,
    end: usize
}

fn parse_input(input: &str) -> Vec<Range> {
    input.split(',')
        .map(|x| {
            let (start, end) = x.split_once('-').unwrap();
            Range { start: start.parse().unwrap(), end: end.parse().unwrap() }
        })
        .collect()
}