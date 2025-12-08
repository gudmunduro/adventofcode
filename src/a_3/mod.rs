use std::fs;

pub fn run_a3() {
    let banks = parse_input(&fs::read_to_string("src/a_3/input.txt").unwrap());

    let mut jolt_sum = 0;
    for bank in &banks {
        let m = find_max_jolt_2(bank, 0, 12);
        println!("Max for bank: {m}");
        jolt_sum += m;
    }

    println!("Max jolt: {jolt_sum}");
}

fn find_max_jolt(bank: &Bank) -> u32 {
    let mut max_jolt = 0;
    for (i, v1) in bank.iter().enumerate().take(bank.len()-1) {
        for v2 in &bank[i+1..] {
            max_jolt = max_jolt.max(v1*10+v2);
        }
    }

    max_jolt
}

fn find_max_jolt_2(bank: &Bank, start: usize, allowed_batteries: usize) -> u64 {
    if allowed_batteries == 0 {
        return 0;
    }

    let max_jolt = bank[start..=bank.len()-allowed_batteries]
        .iter()
        .max_by_key(|j| **j)
        .unwrap();
    let index = bank[start..=bank.len()-allowed_batteries]
        .iter()
        .enumerate()
        .find_map(|(i, j)| {
            if j == max_jolt {
                Some(i)
            }
            else {
                None
            }
        })
        .unwrap();

    *max_jolt as u64 * 10u64.pow(allowed_batteries as u32-1) + find_max_jolt_2(bank, start+index+1, allowed_batteries-1)
}

type Bank = Vec<u32>;
fn parse_input(input: &str) -> Vec<Bank> {
    input
        .split('\n')
        .map(|bank| bank
            .chars()
            .map(|b| b.to_digit(10).unwrap())
            .collect())
        .collect()
}