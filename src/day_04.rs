#![allow(unused, dead_code)]
use crate::utility::*;

pub fn part_1(test_data: TestData) -> String {
    let grid = test_data.get_grid().unwrap();
    log::grid(&grid);
    let mut forkable_paper_rolls = 0;
    for (p, &c) in grid.iter() {
        if c != '@' {
            continue;
        }

        let neighboring_paper_rolls = p.all_neighbors_in(&grid).iter()
            .filter(|&np| grid.get(np).map(|&gc| gc == '@').unwrap_or(false))
            .count();
        if neighboring_paper_rolls < 4 {
            forkable_paper_rolls += 1;
        }
    }

    return forkable_paper_rolls.to_string();
}

pub fn part_2(test_data: TestData) -> String {
    return String::from("Wowies Day 4 Part 2");
}
