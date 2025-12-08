use std::fs;

pub fn run_a4() {
    let grid = parse_input(&fs::read_to_string("src/a_4/input.txt").unwrap());

    let mut count = 0;
    for (r, grid_row) in grid.iter().enumerate() {
        for (c, grid_item) in grid_row.iter().enumerate() {
            if *grid_item == GridItem::Paper && count_adjacent(r, c, &grid) < 4 {
                count += 1;
            }
        }
    }

    println!("Count: {count}");
}

pub fn run_a4_2() {
    let mut grid = parse_input(&fs::read_to_string("src/a_4/input.txt").unwrap());

    let mut total_count = 0;

    loop {
        let mut cleared_papers = Vec::new();
        let mut count = 0;

        for (r, grid_row) in grid.iter().enumerate() {
            for (c, grid_item) in grid_row.iter().enumerate() {
                if *grid_item == GridItem::Paper && count_adjacent(r, c, &grid) < 4 {
                    count += 1;
                    cleared_papers.push((r, c));
                }
            }
        }

        for (r, c) in cleared_papers {
            grid[r][c] = GridItem::Empty;
        }

        total_count += count;
        if count == 0 {
            break;
        }
    }

    println!("Count: {total_count}");
}

fn count_adjacent(r: usize, c: usize, grid: &Grid) -> usize {
    let adjacent = [
        (r+1, c-1), (r+1, c), (r+1, c+1),
        (r, c-1), (r, c+1),
        (r-1, c-1), (r-1, c), (r-1, c+1)
    ];

    adjacent
        .iter()
        .filter(|(r, c)| {
            let item = grid.get(*r)
                .map(|grid_col| grid_col.get(*c))
                .flatten();

            matches!(item, Some(GridItem::Paper))
        })
        .count()
}

type Grid = Vec<Vec<GridItem>>;
#[derive(Eq, PartialEq)]
enum GridItem {
    Paper,
    Empty,
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match &c {
                    '.' => GridItem::Empty,
                    '@' => GridItem::Paper,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}
