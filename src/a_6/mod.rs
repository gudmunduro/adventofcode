use std::fs;

pub fn run_a6() {
    let sheet = parse_input(&fs::read_to_string("src/a_6/input.txt").unwrap());

    use WorksheetItem::*;
    let mut total = 0;
    for col in 0..sheet[0].len() {
        let Symbol(symbol) = &sheet[sheet.len()-1][col] else {
            panic!("Last row is not symbol")
        };

        let res = match symbol {
            '+' => {
                (0..sheet.len()-1)
                    .map(|r| match sheet[r][col] {
                        Number(n) => n,
                        _ => panic!("Not a number")
                    })
                    .sum()
            },
            '*' => {
                (0..sheet.len()-1)
                    .map(|r| match sheet[r][col] {
                        Number(n) => n,
                        _ => panic!("Not a number")
                    })
                    .reduce(|acc, n| acc * n)
                    .unwrap()
            },
            _ => {
                panic!("Invalid symbol")
            }
        };
        
        total += res;
    }
    println!("Sum: {total}");

}

enum WorksheetItem {
    Number(u128),
    Symbol(char)
}

fn parse_input(input: &str) -> Vec<Vec<WorksheetItem>> {
    input.lines()
        .map(|l| l.split_whitespace().map(|n| {
            match n.parse() {
                Ok(n) => WorksheetItem::Number(n),
                Err(_) => WorksheetItem::Symbol(n.chars().next().unwrap())
            }
        }).collect())
        .collect()
}

pub fn run_a6_2() {
    let sheet_lines = parse_input_2(&fs::read_to_string("src/a_6/input.txt").unwrap());

    let total: u128 = sheet_lines.into_iter().map(|(numbers, operator)| {
        match operator {
            '+' => {
                numbers.into_iter().sum()
            },
            '*' => {
                numbers
                    .into_iter()
                    .reduce(|acc, n| acc * n)
                    .unwrap()
            },
            _ => {
                panic!("Invalid symbol")
            }
        }
    }).sum();
    println!("Sum: {total}");

}

fn parse_input_2(input: &str) -> Vec<(Vec<u128>, char)> {
    let grid: Vec<Vec<_>> = input.lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut separator_columns: Vec<_> = grid
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|(i, c)| {
            if matches!(c, '+' | '*') {
                Some(i-1)
            }
            else {
                None
            }
        })
        .collect();
    separator_columns.push(grid[0].len());

    let mut sheet_lines = Vec::new();

    let rows = grid.len();
    for (i, &separator_index) in separator_columns.iter().enumerate() {
        let num_start_col = if i == 0 {
            0
        }
        else {
          separator_columns[i-1]+1
        };

        let mut numbers = Vec::new();
        for c in (num_start_col..separator_index).rev() {
            let number: String = (0..(rows-1)).map(|r| grid[r][c]).filter(|&n| n != ' ').collect();
            match number.parse() {
                Ok(n) => numbers.push(n),
                Err(_) => {}
            }
        }
        let operator = grid[rows-1][num_start_col];

        sheet_lines.push((numbers, operator));
    }

    sheet_lines
}