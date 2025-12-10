use std::fs;
use std::ops::RangeInclusive;
use itertools::Itertools;
use nalgebra::{Vector2, Vector3};
use crate::a_9::Side::{Bottom, Left, Right, Top};

pub fn run_a9_part1() {
    let mut red_tiles = parse_input(&fs::read_to_string("src/a_9/input.txt").unwrap());

    let max_square = red_tiles
        .iter()
        .array_combinations()
        .map(|[s1, s2]| square_size(s1, s2))
        .max()
        .unwrap();
    println!("Max square: {max_square}");
}

pub fn run_a9_part2() {
    let mut red_tiles = parse_input(&fs::read_to_string("src/a_9/input.txt").unwrap());

    let max_square = red_tiles
        .iter()
        .array_combinations()
        .map(|[s1, s2]| Square::new(s1.clone(), s2.clone(), square_size(s1, s2)))
        .sorted_by_key(|s| s.size)
        .rev()
        .filter(|s| is_inside_tile_area(s, &red_tiles))
        .next()
        .unwrap();
    println!("Max square: {}", max_square.size);
}

fn is_inside_tile_area(square: &Square, red_tiles: &Vec<TilePos>) -> bool {
    use Side::*;

    let any_tiles_cross_line = |line: &RangeInclusive<i64>, other_coord: i64, side: Side| red_tiles
        .iter()
        .chain([red_tiles.first().unwrap()].into_iter())
        .tuple_windows()
        .any(|(t1, t2)| {
            let is_within_line_bounds = match side {
                (Top | Bottom) => line.contains(&t1.x) && line.contains(&t2.x),
                Left | Right => line.contains(&t1.y) && line.contains(&t2.y)
            };

            let crosses_line = match side {
                Top | Bottom => {
                    let t_min_y = t1.y.min(t2.y);
                    let t_max_y = t1.y.max(t2.y);
                    match side {
                        Top => t_min_y <= other_coord && other_coord < t_max_y,
                        Bottom => t_min_y < other_coord && other_coord <= t_max_y,
                        _ => panic!()
                    }
                },
                Left | Right => {
                    let t_min_x = t1.x.min(t2.x);
                    let t_max_x = t1.x.max(t2.x);
                    match side {
                        Left => t_min_x <= other_coord && other_coord < t_max_x,
                        Right => t_min_x < other_coord && other_coord <= t_max_x,
                        _ => panic!()
                    }
                }
            };

            is_within_line_bounds && crosses_line
        });

    let c1 = &square.corner_1;
    let c2 = &square.corner_2;

    let top = c1.y.min(c2.y);
    let bottom = c1.y.max(c2.y);
    let left = c1.x.min(c2.x);
    let right = c1.x.max(c2.x);

    let inner_x_line = (left+1)..=(right-1);
    let inner_y_line = (top+1)..=(bottom-1);

    !any_tiles_cross_line(&inner_x_line, top, Top)
        && !any_tiles_cross_line(&inner_x_line, bottom, Bottom)
        && !any_tiles_cross_line(&inner_y_line, left, Left)
        && !any_tiles_cross_line(&inner_y_line, right, Right)
}

fn square_size(tile_1: &TilePos, tile_2: &TilePos) -> i64 {
    ((tile_2.x - tile_1.x).abs() + 1) * ((tile_2.y - tile_1.y).abs() + 1)
}

type TilePos = Vector2<i64>;
fn parse_input(content: &str) -> Vec<TilePos> {
    content
        .lines()
        .map(|l| {
            let vector_numbers = l.split(',').map(|v| v.parse().unwrap()).collect::<Vec<_>>();
            let [x, y] = vector_numbers.as_slice() else {
                panic!("Invalid format of vector in input");
            };
            TilePos::new(*x, *y)
        })
        .collect()
}

enum Side {
    Top,
    Bottom,
    Left,
    Right
}

struct Square {
    corner_1: TilePos,
    corner_2: TilePos,
    size: i64
}

impl Square {
    pub fn new(corner_1: TilePos, corner_2: TilePos, size: i64) -> Self {
        Self {
            corner_1,
            corner_2,
            size
        }
    }
}
