use std::fs;

pub fn run_a7() {
    let (grid, mut beams) = parse_input(&fs::read_to_string("src/a_7/input.txt").unwrap());

    let mut tracking_state = TrackingState::new();
    while beams.iter().any(|b| !b.stopped) {
        let mut new_beams = Vec::new();
        for beam in &mut beams {
            new_beams.extend(update_beam(beam, &grid, &mut tracking_state));
        }
        beams.extend(new_beams);
        beams.retain(|b| !b.stopped);
        beams = merge_beams(&beams);
    }

    print!("World count: {}", tracking_state.split_count+1);
}

fn update_beam(beam: &mut BeamState, grid: &Grid, tracking_state: &mut TrackingState) -> Vec<BeamState> {
    let next_row = beam.row + 1;

    if next_row >= grid.len() {
        beam.stopped = true;
        return vec![];
    }

    use GridItem::*;
    match &grid[next_row][beam.column] {
        Empty => {
            beam.row = next_row;
            vec![]
        }
        Splitter => {
            tracking_state.split_count += beam.beam_count;
            beam.stopped = true;
            vec![
                BeamState::new(next_row, beam.column-1, beam.beam_count),
                BeamState::new(next_row, beam.column+1, beam.beam_count)
            ]
        }
    }
}

fn merge_beams(beams: &Vec<BeamState>) -> Vec<BeamState> {
    let mut positions: Vec<_> = beams.iter().map(|b| (b.row, b.column)).collect();
    positions.sort();
    positions.dedup();

    positions.into_iter()
        .map(|(row, column)| BeamState {
            row,
            column,
            stopped: false,
            beam_count: beams.iter()
                .filter(|b| (b.row, b.column) == (row, column))
                .map(|b| b.beam_count)
                .sum()
        })
        .collect()
}

struct TrackingState {
    split_count: usize
}
impl TrackingState {
    pub fn new() -> Self {
        Self {
            split_count: 0
        }
    }
}

type Grid = Vec<Vec<GridItem>>;
enum GridItem {
    Empty,
    Splitter
}
struct BeamState {
    row: usize,
    column: usize,
    stopped: bool,
    beam_count: usize,
}
impl BeamState {
    pub fn new(row: usize, column: usize, beam_count: usize) -> BeamState {
        BeamState {
            row,
            column,
            stopped: false,
            beam_count
        }
    }
}

fn parse_input(input: &str) -> (Grid, Vec<BeamState>) {
    let mut beams = Vec::new();
    let mut grid = Vec::new();

    for (i, row) in input.split('\n').enumerate() {
        let mut grid_row = Vec::new();

        for (j, col) in row.chars().enumerate() {
            grid_row.push(match col {
                '.' | 'S' => GridItem::Empty,
                '^' => GridItem::Splitter,
                c => panic!("Invalid grid item {c}")
            });
            if col == 'S' {
                beams.push(BeamState::new(i, j, 1));
            }
        }

        grid.push(grid_row);
    }

    (grid, beams)
}