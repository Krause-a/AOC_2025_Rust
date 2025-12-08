#![allow(unused, dead_code)]
use std::collections::{HashMap, HashSet};

use crate::utility::*;

pub fn part_1(test_data: TestData) -> String {
    let mut next_beams : HashSet<usize> = HashSet::new();
    let mut current_beams : HashSet<usize> = HashSet::new();
    let mut lines = test_data.get_lines().unwrap();
    let first_line = lines.next().unwrap();
    let s_pos = first_line.find("S").unwrap();
    next_beams.insert(s_pos);
    let mut split_count = 0;
    for line in lines {
        (next_beams, current_beams) = (current_beams, next_beams);
        log::debug(|| format!("NX: {:?}", next_beams));
        next_beams.clear();
        log::debug(|| format!("LOOP: {:?}", current_beams));
        let mut last_right_beam = usize::max_value();
        log::debug(|| format!("{line}"));
        for (i, ch) in line.chars().enumerate() {
            log::debug(|| format!("i: {i}"));
            if current_beams.contains(&i) {
                if ch == '^' {
                    if i > 0 {
                        if last_right_beam != i - 1 {
                            next_beams.insert(i - 1);
                            log::debug(|| format!("Insert left: {}", i - 1));
                        }
                    }
                    split_count += 1;
                    next_beams.insert(i + 1);
                    log::debug(|| format!("Insert right: {}", i + 1));
                    last_right_beam = i + 1;
                } else {
                    if last_right_beam != i {
                        next_beams.insert(i);
                        log::debug(|| format!("Insert down: {}", i));
                    }
                    else {
                        log::debug(|| format!("Hit last_right_beam value {last_right_beam}"));
                    }
                }
            }
        }
    }

    return split_count.to_string();
}

pub fn part_2(test_data: TestData) -> String {
    let mut next_beams : HashMap<usize, usize> = HashMap::new();
    let mut current_beams : HashMap<usize, usize> = HashMap::new();
    let mut lines = test_data.get_lines().unwrap();
    let first_line = lines.next().unwrap();
    let s_pos = first_line.find("S").unwrap();
    next_beams.insert(s_pos, 1);
    for line in lines {
        (next_beams, current_beams) = (current_beams, next_beams);
        log::debug(|| format!("{:?}", current_beams));
        next_beams.clear();
        for (i, ch) in line.chars().enumerate() {
            let mut path_count = *current_beams.get(&i).unwrap_or(&(0 as usize));
            if path_count > 0 {
                if ch == '^' {
                    if i > 0 {
                        let mut prev_count = *next_beams.get(&(i - 1)).unwrap_or(&(0 as usize));
                        next_beams.insert(i - 1, prev_count + path_count);
                    }
                    next_beams.insert(i + 1, path_count);
                } else {
                    next_beams.insert(i, path_count);
                }
            }
        }
    }
        log::debug(|| format!("{:?}", next_beams));

    return next_beams.values().into_iter().sum::<usize>().to_string();

}
