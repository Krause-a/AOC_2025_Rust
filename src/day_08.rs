#![allow(unused, dead_code)]
use crate::utility::*;

pub fn part_1(test_data: TestData) -> String {
    let desired_connections = if test_data.is_test() {10} else {1000};
    return String::from("Wowies Day 8 Part 1");
}

pub fn part_2(test_data: TestData) -> String {
    return String::from("Wowies Day 8 Part 2");
}

#[derive(Clone, Copy)]
struct Vec {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec {
    pub fn new(x: f32, y: f32, z: f32) -> Vec {
        Vec {
            x,
            y,
            z,
        }
    }

    pub fn distance(self: &Self, other: &Self) -> f32 {
        let d_x = (self.x - other.x).abs();
        let d_y = (self.y - other.y).abs();
        let d_z = (self.z - other.z).abs();
        (d_x.powi(2) + d_y.powi(2) + d_z.powi(2)).sqrt()
    }
}
