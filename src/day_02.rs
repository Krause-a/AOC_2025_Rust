#![allow(unused, dead_code)]
use crate::utility::*;
use std::cmp;

pub fn part_1(test_data: TestData) -> String {
    let mut invalid_ids : Vec<String> = Vec::new();

    let mut words = test_data.get_words(",").unwrap();

    while let Some(range) = words.next() {
        let (low, high) = range.trim().split_once("-").unwrap();
        log::debug(|| format!("Word: {low}-{high}"));

        if low.len() == high.len() && low.len() & 1 != 0 {
            continue; // Don't check odd length IDs
        }

        let mut id = low.to_string();
        while str_num_compare(&id, high) != cmp::Ordering::Greater {
            if id.len() & 1 != 0 {
                id = "1".to_string() + &"0".repeat(id.len());
            }
            let half_len = id.len() / 2;
            let half_id = &id[..half_len];
            let mirror_id = half_id.to_string() + half_id;
            if str_num_compare(&mirror_id, high) != cmp::Ordering::Greater && str_num_compare(&mirror_id, low) != cmp::Ordering::Less {
                invalid_ids.push(half_id.to_string());
                log::debug(|| format!("Low: {low}, High: {high}, Current: {mirror_id}"));
                log::info(|| format!("Invalid ID Found {mirror_id}"));
            }
            let half_id_num = half_id.parse::<usize>().unwrap() + 1;
            id = half_id_num.to_string().repeat(2);
        }
    }

    let mut sum = 0;
    for invalid in invalid_ids {
        let full_id = (invalid.clone() + &invalid).parse::<usize>().unwrap();
        sum += full_id;
    }
    
    return sum.to_string();
}

pub fn part_2(test_data: TestData) -> String {
    return test_data.get_string().unwrap();
}
