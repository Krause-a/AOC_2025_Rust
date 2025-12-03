#![allow(unused, dead_code)]
use std::collections::BinaryHeap;

use crate::utility::*;

pub fn part_1(test_data: TestData) -> String {

    let mut lines = test_data.get_lines().unwrap();
    let mut banks : Vec<u32> = Vec::new();

    for line in lines {
        let mut first_digit_largest = 0;
        let mut second_digit_largest = 0;
        for battery in line[..line.len() - 1].chars().map(|c| c.to_digit(10).unwrap()) {
            if battery > first_digit_largest {
                first_digit_largest = battery;
                second_digit_largest = 0;
            }
            else if battery > second_digit_largest {
                second_digit_largest = battery;
            }
        }
        let last_battery : u32 = line[line.len() - 1..=line.len() - 1].parse().unwrap();
        if last_battery > second_digit_largest {
            second_digit_largest = last_battery;
        }
        banks.push(first_digit_largest * 10 + second_digit_largest);
    }

    return banks.iter().sum::<u32>().to_string();
}

pub fn part_2(test_data: TestData) -> String {

    let mut lines = test_data.get_lines().unwrap();
    let mut banks : Vec<u64> = Vec::new();

    for line in lines {
        let line_digits : Vec<u64> = line[..line.len()].chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
        let line_digit_len = line_digits.len();
        let sub_lines = vec![
            &line_digits[..line_digit_len - 11],
            &line_digits[..line_digit_len - 10],
            &line_digits[..line_digit_len - 9],
            &line_digits[..line_digit_len - 8],
            &line_digits[..line_digit_len - 7],
            &line_digits[..line_digit_len - 6],
            &line_digits[..line_digit_len - 5],
            &line_digits[..line_digit_len - 4],
            &line_digits[..line_digit_len - 3],
            &line_digits[..line_digit_len - 2],
            &line_digits[..line_digit_len - 1],
            &line_digits,
        ];
        let mut selected_digits : Vec<u32> = Vec::new();
        for sub_line in sub_lines {
            log::debug(|| format!("Sub_Line: {:?}", sub_line));
            let mut digits = sub_line.iter();
            let mut last_digit_index = None;
            if !selected_digits.is_empty() {
                last_digit_index = Some(*selected_digits.last().unwrap());
            }
            let last_digit_index = last_digit_index.map(|d| d + 1).unwrap_or(0);
            let mut max_digit = 0;
            let mut max_digit_index = 0;
            for (i, d) in digits.enumerate().skip((last_digit_index) as usize) {
                let d = *d;
                if max_digit < d {
                log::debug(|| format!("{d}"));
                    max_digit = d;
                    max_digit_index = i as u32;
                }
            }
            log::debug(|| format!("Push: {max_digit_index}"));
            selected_digits.push(max_digit_index);
        }
        log::info(|| {
            let selected_batrees = selected_digits.iter().map(|d| line_digits[*d as usize].to_string()).collect::<Vec<String>>().join("");
            format!("Selected: {selected_batrees}")
        });
        banks.push( selected_digits.into_iter().
            rev(). enumerate().
            map(|(i, battery_index)| {
                let battery = line_digits[battery_index as usize];
                log::debug(||format!("{battery}^{i}"));
                battery * (10 as u64).pow(i as u32)
            }).
            sum());
    }

    return banks.iter().sum::<u64>().to_string();
}
