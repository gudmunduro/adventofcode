use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use itertools::Itertools;
use nalgebra::Vector3;

pub fn run_a8_part1() {
    let mut junction_boxes = parse_input(&fs::read_to_string("src/a_8/input.txt").unwrap());
    let shortest_connections = shortest_junction_box_connections(&junction_boxes);

    let mut circuits = HashMap::new();
    for connection in shortest_connections.iter().take(1000) {
        connect_junction_boxes(connection, &mut circuits, &mut junction_boxes);
    }

    let top_3_mul = circuits
        .values()
        .map(|c| c.junction_boxes.len())
        .sorted()
        .rev()
        .take(3)
        .fold(1, |acc, x| acc * x);
    println!("Result: {top_3_mul}");
}

pub fn run_a8_part2() {
    let mut junction_boxes = parse_input(&fs::read_to_string("src/a_8/input.txt").unwrap());
    let shortest_connections = shortest_junction_box_connections(&junction_boxes);

    let mut circuits = HashMap::new();
    for connection in &shortest_connections {
        connect_junction_boxes(connection, &mut circuits, &mut junction_boxes);

        if circuits.iter().next().map(|(k, c)| c.junction_boxes.len() == junction_boxes.len()).unwrap_or(false) {
            let from_pos = &junction_boxes[connection.from].pos;
            let to_pos = &junction_boxes[connection.to].pos;
            println!("Result: (between {} and {}) {}", from_pos.x, to_pos.x, from_pos.x as u64 * to_pos.x as u64);
            break;
        }
    }
}

fn shortest_junction_box_connections(junction_boxes: &Vec<JunctionBox>) -> Vec<Connection> {
    let mut shortest_connections = Vec::new();
    for [j1, j2] in (0..junction_boxes.len()).array_combinations() {
        let junction_1 = &junction_boxes[j1];
        let junction_2 = &junction_boxes[j2];

        shortest_connections.push(Connection::new(j1, j2, (junction_1.pos-junction_2.pos).norm()))
    }

    shortest_connections.sort();
    shortest_connections
}

fn connect_junction_boxes(connection: &Connection, circuits: &mut HashMap<usize, Circuit>, junction_boxes: &mut Vec<JunctionBox>) {
    match (junction_boxes[connection.from].circuit_index, junction_boxes[connection.to].circuit_index) {
        (None, None) => {
            create_circuit(connection.from, connection.to, circuits, junction_boxes);
        },
        (Some(i), None) => {
            connect_to_circuit(i, connection.to, circuits, junction_boxes);
        }
        (None, Some(i)) => {
            connect_to_circuit(i, connection.from, circuits, junction_boxes);
        }
        (Some(index_1), Some(index_2)) if index_1 != index_2 => {
            merge_circuits(index_1, index_2, circuits, junction_boxes);
        }
        (Some(_), Some(_)) => {}
    }
}

fn create_circuit(index_1: usize, index_2: usize, circuits: &mut HashMap<usize, Circuit>, junction_boxes: &mut Vec<JunctionBox>) {
    let mut circuit = Circuit::new();
    circuit.junction_boxes.push(index_1);
    circuit.junction_boxes.push(index_2);
    let index = circuits.keys().max().copied().unwrap_or(0usize)+1;
    circuits.insert(index, circuit);

    junction_boxes[index_1].circuit_index = Some(index);
    junction_boxes[index_2].circuit_index = Some(index);
}

fn connect_to_circuit(circuit_index: usize, junction_box_index: usize, circuits: &mut HashMap<usize, Circuit>, junction_boxes: &mut Vec<JunctionBox>) {
    junction_boxes[junction_box_index].circuit_index = Some(circuit_index);
    circuits.get_mut(&circuit_index).unwrap().junction_boxes.push(junction_box_index);
}

fn merge_circuits(merge_into: usize, merge_from: usize, circuits: &mut HashMap<usize, Circuit>, junction_boxes: &mut Vec<JunctionBox>) {
    for &index in &circuits.get_mut(&merge_from).unwrap().junction_boxes {
        junction_boxes[index].circuit_index = Some(merge_into);
    }
    let merge_from_circuit = circuits.remove(&merge_from).unwrap();
    circuits.get_mut(&merge_into).unwrap().junction_boxes.extend(merge_from_circuit.junction_boxes);
}

fn parse_input(content: &str) -> Vec<JunctionBox> {
    content
        .lines()
        .map(|l| {
            let vector_numbers = l.split(',').map(|v| v.parse().unwrap()).collect::<Vec<f32>>();
            let [x, y, z] = vector_numbers.as_slice() else {
                panic!("Invalid format of vector in input");
            };
            JunctionBox::new(Vector3::new(*x, *y, *z))
        })
        .collect()
}

#[derive(Debug)]
struct Circuit {
    junction_boxes: Vec<usize>
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            junction_boxes: Vec::new()
        }
    }
}

#[derive(Debug)]
struct JunctionBox {
    pos: Vector3<f32>,
    circuit_index: Option<usize>
}

impl JunctionBox {
    pub fn new(pos: Vector3<f32>) -> Self {
        Self {
            pos,
            circuit_index: None
        }
    }
}

impl Display for JunctionBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.pos.x, self.pos.y, self.pos.z)
    }
}

#[derive(Clone, Debug)]
struct Connection {
    from: usize,
    to: usize,
    distance: f32
}

impl Connection {
    pub fn new(from: usize, to: usize, distance: f32) -> Self {
        Self {
            from,
            to,
            distance
        }
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        (self.from, self.to) == (other.from, other.to)
    }
}

impl Eq for Connection {}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> Ordering {
        let dist_int = |dist| (dist * 100.0) as u32;
        dist_int(self.distance).cmp(&dist_int(other.distance))
    }
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
