#![allow(unused, dead_code)]

use crate::utility::*;

pub fn part_1(test_data: TestData) -> String {
    let mut in_ranges = true;
    let mut ranges : Vec<Range> = Vec::new();
    let mut fresh_count = 0;
    for line in test_data.get_lines().unwrap() {
        if line.is_empty() {
            in_ranges = false;
            continue;
        }
        if in_ranges {
            ranges.push(Range::from(line.split_once("-").unwrap()));
        } else {
            let id = line.parse().unwrap();
            fresh_count += if ranges.iter().any(|r| r.contains(id)) {1} else {0};
        }
    }
    return fresh_count.to_string();
}

pub fn part_2(test_data: TestData) -> String {
    let mut ranges : Vec<Range> = Vec::new();
    let mut to_delete_index : Vec<usize> = Vec::new();
    for line in test_data.get_lines().unwrap() {
        if line.is_empty() {
            break;
        }
        let mut range = Range::from(line.split_once("-").unwrap());
        to_delete_index.clear();
        for (i, existing_range) in ranges.iter().enumerate() {
            if range.contains_range(&existing_range) {
                if !to_delete_index.contains(&i) {
                    to_delete_index.push(i);
                }
            }
            if existing_range.contains(range.low) {
                range.low = existing_range.low;
                if !to_delete_index.contains(&i) {
                    to_delete_index.push(i);
                }
            }
            if existing_range.contains(range.high) {
                range.high = existing_range.high;
                if !to_delete_index.contains(&i) {
                    to_delete_index.push(i);
                }
            }
        }
        for &index in to_delete_index.iter().rev() {
            ranges.swap_remove(index);
        }
        ranges.push(range);
    }
    log::info(|| format!("Final Ranges: {:?}", ranges));
    let fresh_count : usize = ranges.iter().map(|r| r.total_span()).sum();
    return fresh_count.to_string();
}

struct Range {
    low: usize,
    high: usize,
}

impl Range {
    fn new(a: usize, b: usize) -> Range {
        let low = a.min(b);
        let high = a.max(b);
        Range { low, high }
    }
    fn from(strs: (&str, &str)) -> Range {
        Range::new(strs.0.parse().unwrap(), strs.1.parse().unwrap())
    }
    fn is_valid(self: &Self) -> bool {
        self.low < self.high
    }
    fn contains(self: &Self, num: usize) -> bool {
        self.low <= num && num <= self.high
    }
    fn contains_range(self: &Self, other: &Range) -> bool {
        self.low <= other.low && self.high >= other.high
    }
    fn total_span(self: &Self) -> usize {
        self.high - self.low + 1
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.low, self.high)
    }
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.low, self.high)
    }
}
