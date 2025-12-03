#![allow(unused, dead_code)]
use crate::utility::*;
use std::{cmp, collections::HashSet};

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
        while id.num_le(high) {
            if id.len() & 1 != 0 {
                id = "1".to_string() + &"0".repeat(id.len());
            }
            let half_len = id.len() / 2;
            let half_id = &id[..half_len];
            let mirror_id = half_id.to_string() + half_id;
            if mirror_id.num_le(high) && mirror_id.num_ge(low) {
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
    let mut invalid_ids : HashSet<String> = HashSet::new();
    let mut sum = 0;
    let mut words = test_data.get_words(",").unwrap();

    while let Some(range) = words.next() {
        let (low, high) = range.trim().split_once("-").unwrap();

        log::debug(|| format!("Word: {low}-{high}"));

        let mut id = low.to_string();
        while id.num_le(high) {
            // This walks every id in the range.
            let next_id = (id.parse::<usize>().unwrap() + 1).to_string();
            for repeats in 2..=id.len() {
                if id.len() % repeats != 0 {
                    continue;
                }
                let frac = id.len() / repeats;
                let id_frac = id[..frac].to_string();

                log::debug(|| format!("Repeats: {repeats}, Frac: {id_frac}, ID: {id}"));

                let mirror_id = id_frac.repeat(repeats);
                if !invalid_ids.contains(&mirror_id) && mirror_id.num_le(high) && mirror_id.num_ge(low)  {

                    sum += mirror_id.parse::<usize>().unwrap();

                    log::debug(|| format!("Low: {low}, High: {high}, Current: {mirror_id}"));
                    log::info(|| format!("Invalid ID Found {mirror_id}"));

                    invalid_ids.insert(mirror_id);
                }
            }
            id = next_id;
        }
    }
    log::info(|| format!("Total stored invalid ids: {}", invalid_ids.len()));

    return sum.to_string();
}
