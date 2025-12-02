#![allow(unused, dead_code)]
use crate::utility::*;

pub fn part_1(test_data: TestData) -> String {
    let mut pos : isize = 50;
    let mut lines = test_data.get_lines().unwrap();
    let mut rotations = lines.map(|ns| if ns.starts_with("R") {1}else{-1} * ns[1..].parse::<isize>().unwrap());
    let mut zeros = 0;

    while let Some(rotation) = rotations.next() {
        pos = (pos + rotation) % 100;
        if pos < 0 {
            pos += 100
        }
        if pos == 0 {
            zeros += 1
        }
    }

    return zeros.to_string();
}

pub fn part_2(test_data: TestData) -> String {
    let mut pos : isize = 50;
    let mut lines = test_data.get_lines().unwrap();
    let mut rotations = lines.map(|ns| if ns.starts_with("R") {1}else{-1} * ns[1..].parse::<isize>().unwrap());
    let mut zeros = 0;
    while let Some(rotation) = rotations.next() {
        let mut pos_was_zero = pos == 0;
        pos += rotation;
        while pos < 0 {
            pos += 100;
            zeros += 1;
            if pos_was_zero {
                zeros -= 1;
                pos_was_zero = false;
            }
        }
        if pos == 0 {
            zeros += 1
        }
        while pos >= 100 {
            pos -= 100;
            zeros += 1;
        }
        log::debug(|| format!("{:04}: {pos} {}", zeros, rotation));
    }
    return zeros.to_string();
}

fn get_rotations(test_data: TestData)  -> impl Iterator<Item = isize> {
    let mut lines = test_data.get_lines().unwrap();
    lines.map(|ns| if ns.starts_with("R") {1}else{-1} * ns[1..].parse::<isize>().unwrap())
}
