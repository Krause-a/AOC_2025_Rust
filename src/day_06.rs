#![allow(unused, dead_code)]
use crate::utility::*;

pub fn part_1(test_data: TestData) -> String {
    let mut num_lines : Vec<Vec<usize>> = Vec::new();
    let mut operations : Vec<Operation> = Vec::new();

    for line in test_data.get_lines().unwrap() {
        let mut line_of_nums = Vec::new();
        for str_num in line.split_whitespace() {
            if let Ok(num) = str_num.parse() {
                line_of_nums.push(num);
            }
            else {
                operations.push(match str_num {
                    "+" => Operation::Add,
                    "*" => Operation::Multiply,
                    _ => unimplemented!(),
                });
            }
        }
        if !line_of_nums.is_empty() {
            num_lines.push(line_of_nums);
        }
    }

    let range = 0..operations.len();
    let mut problem_numbers : Vec<usize> = Vec::new();
    let mut sum = 0;
    for i in range {
        problem_numbers.clear();
        for num_line in num_lines.iter() {
            problem_numbers.push(num_line[i]);
        }
        let op = operations[i];
        sum += op.apply(problem_numbers.as_slice())
    }
    return sum.to_string();
}

pub fn part_2(test_data: TestData) -> String {
    let mut num_lines : Vec<Vec<&str>> = Vec::new();
    let mut operations : Vec<Operation> = Vec::new();

    let mut lines : Vec<String> = test_data.get_lines().unwrap().collect();
    let op_line = lines.pop().unwrap();
    for str_num in op_line.split_whitespace() {
        match str_num {
            "+" => {
                operations.push(Operation::Add);
            },
            "*" => {
                operations.push(Operation::Multiply);
            },
            _ => unimplemented!(),
        }
    }

    let mut op_index = 0;
    let mut problem_numbers : Vec<usize> = Vec::new();
    let mut sum = 0;
    for i in 0..lines[0].len() {
        let mut num_str = String::new();
        for line in lines.iter() {
            num_str.push_str(&line[i..=i]);
        }
        let num_str = num_str.trim();
        if num_str.is_empty() {
            sum += operations[op_index].apply(&problem_numbers);
            op_index += 1;
            problem_numbers.clear();
        }
        else {
            problem_numbers.push(num_str.parse().unwrap());
        }
    }
    sum += operations.last().unwrap().apply(&problem_numbers);
    return sum.to_string();
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn apply(self: Self, numbers : &[usize]) -> usize {
        if self == Operation::Add {
            numbers.iter().copied().reduce(|acc, item| acc + item).unwrap()
        }
        else {
            let a : &i32 = &5;
            let b : i32 = *a;
            numbers.iter().copied().reduce(|acc, item| acc * item).unwrap()
        }
    }
}
