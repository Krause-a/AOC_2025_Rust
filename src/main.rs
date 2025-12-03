mod utility;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
use std::{fs, path::PathBuf};

fn main() {
    // 1. Read in the inputs
    // Inputs look like
    // 5 1 0 ERROR
    // 2 2 INFO
    // 2 1 1
    // 4 2
    let mut args = std::env::args();
    let _called_exec = args.next();
    let day : usize = args.next().expect("Missing DAY argument").parse().expect("DAY argument must be a number");
    let part : usize = args.next().expect("Missing PART argument").parse().expect("PART argument must be a number");
    let mut use_test_set = true;
    let mut debug_string = None;
    if let Some(third_arg) = args.next() {
        if let Ok(test_set_num) = third_arg.parse::<usize>() {
            use_test_set = test_set_num == 1;
        }
        else {
            debug_string = Some(third_arg);
        }
    }
    if let Some(fourth_arg) = args.next() {
        assert!(debug_string.is_none());
        debug_string = Some(fourth_arg);
    }

    utility::log::parse_and_set_log_level(debug_string.unwrap_or("".to_string()).as_str());

    // 2. Select the correct data file
    let mut data_file_name = format!("data/{day:02}");
    if use_test_set {
        data_file_name += format!("_{part}_test").as_str();
    }
    let file_metadata = fs::metadata(data_file_name.clone()).expect("Couldn't open file metadata");
    if file_metadata.len() < 5 && use_test_set {
        data_file_name = format!("data/{day:02}_1_test");
    }
    let test_data = utility::TestData::new(PathBuf::from(data_file_name));

    // 3. Run correct day part
    let start = std::time::Instant::now();
    let answer = run_day_part(day, part, test_data);
    let duration = start.elapsed();

    println!("The answer is: {answer}");
    println!("Found in ({:?})", duration);

}
fn run_day_part(day: usize, part: usize, test_data: utility::TestData) -> String {
    match day {
        1 => match part {
            1 => day_01::part_1(test_data),
            2 => day_01::part_2(test_data),
            _ => unimplemented!(),
        }
        2 => match part {
            1 => day_02::part_1(test_data),
            2 => day_02::part_2(test_data),
            _ => unimplemented!(),
        }
        3 => match part {
            1 => day_03::part_1(test_data),
            2 => day_03::part_2(test_data),
            _ => unimplemented!(),
        }
        4 => match part {
            1 => day_04::part_1(test_data),
            2 => day_04::part_2(test_data),
            _ => unimplemented!(),
        }
        5 => match part {
            1 => day_05::part_1(test_data),
            2 => day_05::part_2(test_data),
            _ => unimplemented!(),
        }
        6 => match part {
            1 => day_06::part_1(test_data),
            2 => day_06::part_2(test_data),
            _ => unimplemented!(),
        }
        7 => match part {
            1 => day_07::part_1(test_data),
            2 => day_07::part_2(test_data),
            _ => unimplemented!(),
        }
        8 => match part {
            1 => day_08::part_1(test_data),
            2 => day_08::part_2(test_data),
            _ => unimplemented!(),
        }
        9 => match part {
            1 => day_09::part_1(test_data),
            2 => day_09::part_2(test_data),
            _ => unimplemented!(),
        }
        10 => match part {
            1 => day_10::part_1(test_data),
            2 => day_10::part_2(test_data),
            _ => unimplemented!(),
        }
        11 => match part {
            1 => day_11::part_1(test_data),
            2 => day_11::part_2(test_data),
            _ => unimplemented!(),
        }
        12 => match part {
            1 => day_12::part_1(test_data),
            2 => day_12::part_2(test_data),
            _ => unimplemented!(),
        }
        _ => unimplemented!(),
    }
}
