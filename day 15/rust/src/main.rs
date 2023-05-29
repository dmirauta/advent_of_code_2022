use regex::Regex;
use std::{collections::HashSet, fs};

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn lev_dist_to(&self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct SensorData {
    sensor_pos: Point,
    beacon_pos: Point,
    empty_zone_radius: i32,
}

fn parse(contents: &String) -> Vec<SensorData> {
    lazy_static! {
        static ref REG: Regex =
            Regex::new(r"Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)")
                .expect("Regex compile failure");
    }

    let mut data: Vec<SensorData> = vec![];

    for line in contents.lines() {
        if let Some(cap) = REG.captures(line) {
            let sensor_pos = Point {
                x: cap.get(1).unwrap().as_str().parse().unwrap(),
                y: cap.get(2).unwrap().as_str().parse().unwrap(),
            };
            let beacon_pos = Point {
                x: cap.get(3).unwrap().as_str().parse().unwrap(),
                y: cap.get(4).unwrap().as_str().parse().unwrap(),
            };
            data.push(SensorData {
                sensor_pos,
                beacon_pos,
                empty_zone_radius: sensor_pos.lev_dist_to(beacon_pos),
            });
        }
    }

    data
}

fn part_1(data: &Vec<SensorData>, row: i32) {
    let mut free_positions: HashSet<i32> = HashSet::new();

    let mut beacons_in_row = HashSet::new();
    for d in data {
        if d.beacon_pos.y == row {
            beacons_in_row.insert(d.beacon_pos.x);
        }
    }

    for d in data {
        let partial_dist = (row - d.sensor_pos.y).abs(); // distance to sensor projected to row
        for i in 0..=d.empty_zone_radius - partial_dist {
            free_positions.insert(d.sensor_pos.x - i);
            free_positions.insert(d.sensor_pos.x + i);
        }
    }

    dbg!(free_positions.len() - beacons_in_row.len());
}

fn part_2_row_scan(data: &Vec<SensorData>, row: i32, max_coord: i32) -> HashSet<i32> {
    let mut possible_positions: HashSet<i32> = HashSet::from_iter(0..=max_coord);

    for d in data {
        let partial_dist = (row - d.sensor_pos.y).abs(); // distance to sensor projected to row
        for i in 0..=d.empty_zone_radius - partial_dist {
            possible_positions.remove(&(d.sensor_pos.x - i));
            possible_positions.remove(&(d.sensor_pos.x + i));
        }
    }

    possible_positions
}

fn part_2(data: &Vec<SensorData>, max_coord: i32) {
    let mut possible_positions: HashSet<Point> = HashSet::new();
    for row in 0..=max_coord {
        for x in part_2_row_scan(&data, row, max_coord) {
            possible_positions.insert(Point { x, y: row });
        }
    }

    assert!(possible_positions.len() == 1);

    let beacon_pos = possible_positions.drain().next().unwrap();
    let tunning_frequency = beacon_pos.x * 4_000_000 + beacon_pos.y;

    dbg!(tunning_frequency);
}

static TEST_INPUT_PATH: &str = "../test_input";
static INPUT_PATH: &str = "../input";

fn main() {
    let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let tdata = parse(&tcontents);
    let data = parse(&contents);

    // part_1(&tdata, 10);
    // part_1(&data, 2_000_000);

    part_2(&tdata, 20);
    part_2(&data, 4_000_000);
}
