#![allow(unused, dead_code)]
use std::collections::HashMap;

use crate::utility::*;

pub fn part_1(test_data: TestData) -> String {
    let grid = test_data.get_grid().unwrap();
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
    let grid = test_data.get_grid().unwrap();
    let mut roll_grid : HashMap<Point, usize> = HashMap::with_capacity(grid.len());
    for (p, &c) in grid.iter() {
        if c != '@' {
            continue;
        }
        for n in p.all_neighbors_in(&grid) {
            if let Some(&c) = grid.get(&n) {
                if c != '@' {
                    continue;
                }
            }
            let mut roll_count = 1;
            if let Some(existing) = roll_grid.get(&n) {
                roll_count += existing;
            }
            roll_grid.insert(n.clone(), roll_count);
        }
    }

    let style = "recursive";
    let mut forked_paper_rolls : usize = 0;
    if style == "iterative" {
        let mut last_loop_forked_paper_rolls : usize = 1; // 1 is just an initial miss value
        while last_loop_forked_paper_rolls != forked_paper_rolls {
            last_loop_forked_paper_rolls = forked_paper_rolls;
            let forkables = roll_grid.iter().filter(|(k, v)| **v < 4).map(|(k, v)| k).cloned().collect::<Vec<_>>();
            for forkable_roll in forkables {
                forked_paper_rolls += 1;
                roll_grid.remove(&forkable_roll);
                for n in forkable_roll.all_neighbors_in(&roll_grid) {
                    if !roll_grid.contains_key(&n) {
                        continue;
                    }
                    let mut roll_count = 0;
                    if let Some(&existing) = roll_grid.get(&n) {
                        roll_count = 1.max(existing) - 1;
                    }
                    roll_grid.insert(n.clone(), roll_count);
                }
            }
        }
    }
    else if style == "recursive" {
        // Every roll that has less than four neighbors is the starting point of a "fire"
        let initial_len = roll_grid.len();
        let forkables = roll_grid.iter().filter(|(k, v)| **v < 4).map(|(k, v)| k).cloned().collect::<Vec<_>>();
        log::grid(&roll_grid);
        for f in forkables {
            if roll_grid.contains_key(&f) {
                remove_rolls(&mut roll_grid, f);
            }
        }
        forked_paper_rolls = initial_len - roll_grid.len();
    }


    log::grid(&roll_grid);

    return forked_paper_rolls.to_string();
}

fn remove_rolls(grid: &mut HashMap<Point, usize>, p: Point) {
    grid.remove(&p);
    for n in p.all_neighbors_in(&grid) {
        if let Some(count) = grid.get_mut(&n).map(|v| (*v)-1).to_owned() {
            if count < 4 {
                remove_rolls(grid, n);
            }
            else {
                grid.insert(n, count);
            }
        }
    }
}
