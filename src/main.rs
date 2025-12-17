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
use std::{collections::HashMap, fs, io::Read, path::PathBuf};
use utility::StrNum;

fn main() {
    // 1. Read in the inputs
    // Inputs look like
    // 5 1 0 ERROR
    // 2 2 INFO
    // 2 1 1
    // 4 2
    // 4
    let mut args = std::env::args();
    let _called_exec = args.next();
    let day : usize = args.next().expect("Missing DAY argument").parse().expect("DAY argument must be a number");
    let mut part : usize = 0;
    if let Some(part_str) = args.next() {
        part = part_str.parse().expect("PART argument must be a number");
    }
    let mut use_test_set = part != 0;
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
    let mut test_data = utility::TestData::new(PathBuf::from(data_file_name.clone()), day, part, use_test_set);

    // 3. Run correct day part
    let start = std::time::Instant::now();
    println!();
    if part == 0 {
        let answer_1 = run_day_part(day, 1, test_data);
        test_data = utility::TestData::new(PathBuf::from(data_file_name), day, part, use_test_set);
        println!("The part 1 answer is: {answer_1}");
        let comp = compare_to_answer(&answer_1, day, 1, use_test_set);
        if comp.len() > 0 {
            println!("{}", comp);
        }
        println!();
        let answer_2 = run_day_part(day, 2, test_data);
        println!("The part 2 answer is: {answer_2}");
        let comp = compare_to_answer(&answer_2, day, 2, use_test_set);
        if comp.len() > 0 {
            println!("{}", comp);
        }
        println!();
    }
    else {
        let answer = run_day_part(day, part, test_data);
        println!("The part {part} answer is: {answer}");
        let comp = compare_to_answer(&answer, day, part, use_test_set);
        if comp.len() > 0 {
            println!("{}", comp);
        }
        println!();
    }
    let duration = start.elapsed();

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

fn compare_to_answer(maybe_answer: &str, day: usize, part: usize, is_test: bool) -> String {
    let mut answers : HashMap<String, isize> = HashMap::new();
    let mut all_answers = String::new();
    fs::File::open("./data/answers").unwrap().read_to_string(&mut all_answers).unwrap();
    let mut in_day = false;
    let day_str = format!("day_{}", day);
    for line in all_answers.lines().filter(|l| l.trim().len() > 0) {
        if line.starts_with(&day_str) {
            in_day = true;
            continue;
        }
        else if line.starts_with("day_") {
            in_day = false;
        }
        if !in_day {
            continue;
        }
        let (key, value) = line.split_once("=").unwrap();
        answers.insert(key.to_string(), value.parse().unwrap());
    }

    let part_str = format!("part_{}", part);
    let mut low_key = String::new();
    let mut high_key = String::new();
    let mut exact_key = part_str.clone();
    if is_test {
        exact_key += "_test";
    }
    else {
        low_key = part_str.clone() + "_low";
        high_key = part_str.clone() + "_high";
    }

    if answers.contains_key(&exact_key) {
        let answer = *answers.get(&exact_key).unwrap();
        if maybe_answer.is_num() {
            let num_maybe_answer = maybe_answer.parse().unwrap();
            if answer > num_maybe_answer {
                return format!("Given value ({}) is less than answer ({})", maybe_answer, answer);
            }
            else if answer < num_maybe_answer {
                return format!("Given value ({}) is greater than answer ({})", maybe_answer, answer);
            }
            else {
                return format!("Given value ({}) is equal to answer", maybe_answer);
            }
        }
        else {
            if maybe_answer == answer.to_string() {
                return format!("Given value ({}) is equal to answer", maybe_answer);
            }
            else {
                return format!("Given value ({}) is NOT equal to answer ({})", maybe_answer, answer);
            }
        }
    }
    else {
        if maybe_answer.is_num() {
            let num_maybe_answer = maybe_answer.parse::<isize>().unwrap();
            if answers.contains_key(&low_key) {
                let low = *answers.get(&low_key).unwrap();
                if num_maybe_answer <= low {
                    return format!("Given value ({}) is less than or equal to current low ({})", maybe_answer, low);
                }
            }
            if answers.contains_key(&high_key) {
                let high = *answers.get(&high_key).unwrap();
                if num_maybe_answer >= high {
                    return format!("Given value ({}) is greater than or equal to current high ({})", maybe_answer, high);
                }
            }
        }
    }
    return String::new();
}
