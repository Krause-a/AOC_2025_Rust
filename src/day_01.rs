#![allow(unused, dead_code)]
use crate::utility::*;

pub fn part_1(test_data: TestData) -> String {
    let mut pos : isize = 50;
    let mut lines = test_data.get_lines().unwrap();
    let mut zeros = 0;
    while let Some(line) = lines.next() {
        if line.starts_with("R") {
            pos += line[1..].parse::<isize>().unwrap();
        }
        else {
            pos -= line[1..].parse::<isize>().unwrap();
        }
        while pos < 0 {
            pos += 100
        }
        if pos >= 100 {
            pos %= 100
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
    let mut zeros = 0;
    while let Some(line) = lines.next() {
        let mut pos_was_zero = pos == 0;
        if line.starts_with("R") {
            pos += line[1..].parse::<isize>().unwrap();
        }
        else {
            pos -= line[1..].parse::<isize>().unwrap();
        }
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
        log::debug(|| format!("{:04}: {pos} {}", zeros, line));
    }
    return zeros.to_string();
}
