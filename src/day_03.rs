#![allow(unused, dead_code)]
use crate::utility::*;
use std::iter;

pub fn part_1(test_data: TestData) -> String {
    return calculate_banks_sum(test_data, 2).to_string();
}

pub fn part_2(test_data: TestData) -> String {
    return calculate_banks_sum(test_data, 12).to_string();
}

fn calculate_banks_sum(test_data: TestData, battery_count: usize) -> usize {
    return test_data.get_lines().unwrap().map(|l| highest_digit_sequence(l, battery_count)).sum();
}

fn highest_digit_sequence(line: String, desired_digits: usize) -> usize {
    let line_digits : Vec<usize> = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let line_digit_len = line_digits.len();
    let sub_lines : Vec<_> = (1..desired_digits)
        .rev()
        .map(|i| &line_digits[..line_digit_len - i])
        .chain(iter::once(line_digits.as_slice()))
        .collect();
    let mut selected_digits : Vec<usize> = Vec::new();
    for sub_line in sub_lines {
        log::debug(|| format!("Sub_Line: {:?}", sub_line));
        let mut last_digit_index = None;
        if !selected_digits.is_empty() {
            last_digit_index = Some(*selected_digits.last().unwrap());
        }
        let last_digit_index = last_digit_index.map(|d| d + 1).unwrap_or(0);
        let mut max_digit = 0;
        let mut max_digit_index = 0;
        for (i, &d) in sub_line.iter().enumerate().skip(last_digit_index) {
            if max_digit < d {
                max_digit = d;
                max_digit_index = i;
            }
            if max_digit == 9 {
                break;
            }
        }
        log::debug(|| format!("Push: {max_digit_index}"));
        selected_digits.push(max_digit_index);
    }
    log::info(|| {
        let selected_batrees = selected_digits.iter().map(|d| line_digits[*d].to_string()).collect::<Vec<String>>().join("");
        format!("Selected: {selected_batrees}")
    });

    selected_digits.into_iter().rev().enumerate().map(|(i, battery_index)| {
        let battery = line_digits[battery_index];
        log::debug(|| format!("{battery}^{i}"));
        battery * (10 as usize).pow(i as u32)
    }).sum()
}
