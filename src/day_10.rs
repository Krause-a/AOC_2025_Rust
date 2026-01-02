#![allow(unused, dead_code)]
use std::collections::{HashMap, VecDeque};

use crate::utility::*;

pub fn part_1(test_data: TestData) -> String {
    for line in test_data.get_lines().unwrap() {
        let mut line_parts = line.split_whitespace();
        let mut indicator = Indicator::from_string(line_parts.next().unwrap());
        let mut buttons : Vec<Button> = Vec::new();
        let mut trailing_entry = "";
        for line_part in line_parts {
            if trailing_entry.len() != 0 {
                buttons.push(Button::from_string(trailing_entry));
            }
            trailing_entry = line_part;
        }
        let joltage_string = trailing_entry;
    }

    // Notes about button pressing
    // The order the buttons are pressed in does not matter
    // A button press matters all the time
    //
    // How about we try A*?

    unimplemented!();
}

fn a_star(start:&Indicator) {
    let mut open_set = VecDeque::new();
    open_set.push(start);

    let mut g_score = HashMap::new();
    g_score.insert(start.lights, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start.lights, start.distance());

    while
}

#[derive(Debug, PartialEq, Eq)]
struct Indicator {
    goal_lights : u16,
    lights : u16,
    size: u8,
}

impl Indicator {
    fn new(goal_lights:u16, size:u8) -> Indicator {
        Indicator { goal_lights, lights: 0, size }
    }

    fn from_string(s : &str) -> Indicator {
        let s = &s[1..s.len()-1];
        let mut goal_lights = 0;
        for (i, ch) in s.chars().enumerate() {
            if ch == '#' {
                goal_lights |= 1u16.wrapping_shl(i as u32);
            }
        }

        Indicator::new(goal_lights, s.len() as u8)
    }

    fn with_button(self: &Self, button: &Button) -> Indicator {
        let mut lights = self.lights;
        for wire in button.wires.iter() {
            lights ^= 1u16.wrapping_shl(*wire as u32);
        }
        Indicator {
            goal_lights: self.goal_lights,
            lights,
            size: self.size,
        }
    }

    fn is_at_goal(self: &Self) -> bool {
        self.goal_lights == self.lights
    }

    fn distance(self: &Self) -> usize {
        let mut dis = 0;
        for i in 0..self.size {
            let mask = 1u16.wrapping_shl(i as u32);
            if (self.lights & mask) != (self.goal_lights & mask) {
                dis += 1;
            }
        }
        dis
    }
}

#[derive(Debug)]
struct Button {
    wires : Vec<usize>,
}

impl Button {
    fn new(wires : Vec<usize>) -> Button {
        Button {
            wires
        }
    }

    fn from_string(s : &str) -> Button {
        let s = &s[1..s.len()-1];
        let mut v = Vec::new();
        for num_str in s.split(',') {
            v.push(num_str.parse().unwrap());
        }
        Button::new(v)
    }
}

pub fn part_2(test_data: TestData) -> String {
    return String::from("Wowies Day 10 Part 2");
}
