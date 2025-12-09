#![allow(unused, dead_code)]
use std::{collections::BinaryHeap, fmt::Debug, str::FromStr, string::ParseError};

use crate::utility::*;

pub fn part_1(test_data: TestData) -> String {
    let desired_connections = if test_data.is_test() {10} else {1000};
    let mut junctions : Vec<Junction> = Vec::new();
    
    for line in test_data.get_lines().unwrap() {
        let coord = Point3::from_str(&line).unwrap();
        junctions.push(Junction::new(coord));
    }

    // TODO: Make this a min heap based on connection distances
    let mut connections : BinaryHeap<Connection> = BinaryHeap::new();
    for j_1 in junctions.iter() {
        for j_2 in junctions.iter() {
            if j_1 == j_2 {
                continue;
            }
            connections.push(Connection::new(
                    j_1,
                    j_2
            ));
        }
    }
    log::debug(|| format!("{:?}", connections.pop().unwrap()));

    return "0".to_string();
}

pub fn part_2(test_data: TestData) -> String {
    return String::from("Wowies Day 8 Part 2");
}

// TODO: Consider moving into utility file
#[derive(Clone, Copy, Debug, PartialEq)]
struct Point3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3 {
    fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 {
            x,
            y,
            z,
        }
    }

    fn distance(self: &Self, other: &Self) -> f32 {
        let d_x = (self.x - other.x).abs();
        let d_y = (self.y - other.y).abs();
        let d_z = (self.z - other.z).abs();
        (d_x.powi(2) + d_y.powi(2) + d_z.powi(2)).sqrt()
    }
}

struct ParsePoint3Error {}
impl Debug for ParsePoint3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse string into Point3")
    }
}

impl FromStr for Point3 {
    type Err = ParsePoint3Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits : Vec<_> = s.split(",").collect();
        if splits.len() != 2 && splits.len() != 3 {
            Err(ParsePoint3Error {})
        }
        else {
            let x = splits[0].parse().map_err(|_| ParsePoint3Error{})?;
            let y = splits[1].parse().map_err(|_| ParsePoint3Error{})?;
            let z = splits.get(2).map(|s| s.parse().map_err(|_| ParsePoint3Error{})).unwrap_or(Ok(0.0))?;
            Ok(Point3 {
                x,
                y,
                z,
            })
        }
    }
}

#[derive(Debug)]
struct Junction {
    v: Point3,
    circut_member: usize,
}

impl Junction {
    fn new(v: Point3) -> Junction {
        Junction {
            v,
            circut_member: 1,
        }
    }
}

impl PartialEq for Junction {
    fn eq(&self, other: &Self) -> bool {
        self.v.eq(&other.v)
    }
}

#[derive(Debug)]
struct Connection<'a> {
    junc_1: &'a Junction,
    junc_2: &'a Junction,
    distance: f32,
}

impl Connection <'_> {
    fn new<'a>(j_1: &'a Junction, j_2: &'a Junction) -> Connection<'a> {
        Connection {
            junc_1: j_1,
            junc_2: j_2,
            distance: j_1.v.distance(&j_2.v)
        }
    }
}

impl PartialEq for Connection <'_> {
    fn eq(&self, other: &Self) -> bool {
        self.junc_1.eq(&other.junc_1) &&
        self.junc_2.eq(&other.junc_2)
    }
}
impl Eq for Connection <'_> {}

// Comparrisons are reversed to make it act as a min heap when used in a heap.
impl PartialOrd for Connection <'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for Connection <'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.total_cmp(&self.distance)
    }
}
