#![allow(unused, dead_code)]
use std::{collections::BinaryHeap, str::FromStr};

use crate::utility::*;

pub fn part_1(test_data: TestData) -> String {
    let points : Vec<_> = test_data.get_lines().unwrap().map(|l| Point::from_str(&l).unwrap()).collect();
    let mut rects = BinaryHeap::new();
    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i) {
            if p1 == p2 {
                continue
            }
            rects.push(Rect::new(p1.clone(), p2.clone()));
        }
    }
    let rect = rects.pop().unwrap();
    return rect.area().to_string();
}

pub fn part_2(test_data: TestData) -> String {
    return String::from("Wowies Day 9 Part 2");
}

#[derive(Debug)]
pub struct Rect {
    p1: Point,
    p2: Point,
}

impl Rect {
    pub fn new(p1: Point, p2: Point) -> Rect {
        Rect {
            p1,
            p2,
        }
    }
    fn area(self: &Self) -> usize {
        (1 + self.p1.x.abs_diff(self.p2.x)) * (1 + self.p1.y.abs_diff(self.p2.y))
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.p1.eq(&other.p1) && self.p2.eq(&other.p2)
    }
}

impl Eq for Rect {}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rect {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // I might need to invert this to get proper heap ordering
        self.area().cmp(&other.area())
    }
}
