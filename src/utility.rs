#![allow(unused, dead_code)]

use std::{cmp, collections::HashMap, fs, io::{self, BufRead, Bytes, Lines, Read}};

pub struct TestData {
    file_path: std::path::PathBuf,
    day: usize,
    part: usize,
    test: bool,
    answers: Option<HashMap<String, isize>>,
}

impl TestData {
    pub fn new(file_path: std::path::PathBuf, day: usize, part: usize, test: bool) -> TestData {
        let mut test_data = TestData { file_path: file_path, day: day, part: part, test: test, answers: None };
        let mut answers = HashMap::new();
        let mut all_answers = String::new();
        let fh = fs::File::open("./data/answers").unwrap().read_to_string(&mut all_answers).unwrap();
        let mut in_day = false;
        let day_str = format!("day_{}", test_data.day);
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
        test_data.answers = Some(answers);
        test_data
    }
    fn open_file(self: &Self) -> Result<fs::File, io::Error> {
        fs::File::open(&self.file_path)
    }
    pub fn get_string(self: &Self) -> Result<String, io::Error> {
        let mut f = self.open_file()?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    pub fn get_lines(self: &Self) -> Result<AssumeLines<io::BufReader<fs::File>>, io::Error> {
        let f = self.open_file()?;
        let reader = io::BufReader::new(f);
        Ok(reader.assume_lines())
    }
    pub fn get_chars(self: &Self) -> Result<AssumeChars<io::BufReader<fs::File>>, io::Error> {
        let f = self.open_file()?;
        let reader = io::BufReader::new(f);
        Ok(reader.assume_chars())
    }
    pub fn get_words(self: &Self, separator: &'static str) -> Result<AssumeWords<io::BufReader<fs::File>>, io::Error> {
        let f = self.open_file()?;
        let reader = io::BufReader::new(f);
        Ok(reader.assume_words(separator))
    }
    pub fn compare_answer(self: &Self, check_answer: isize) -> String {
        let answers = self.answers.as_ref().unwrap();
        let part_str = format!("part_{}", self.part);
        let mut low_key = String::new();
        let mut high_key = String::new();
        let mut exact_key = part_str.clone();
        if self.test {
            exact_key += "_test";
        }
        else {
            low_key = part_str.clone() + "_low";
            high_key = part_str.clone() + "_high";
        }

        if answers.contains_key(&exact_key) {
            let answer = *answers.get(&exact_key).unwrap();
            if answer > check_answer {
                return format!("Given value ({}) is less than answers ({})", check_answer, answer);
            }
            else if answer < check_answer {
                return format!("Given value ({}) is greater than answers ({})", check_answer, answer);
            }
            else {
                return format!("Given value ({}) is equal to answer", check_answer);
            }
        }
        else {
            if answers.contains_key(&low_key) {
                let low = *answers.get(&low_key).unwrap();
                if check_answer <= low {
                    return format!("Given value ({}) is less than current low ({})", check_answer, low);
                }
            }
            if answers.contains_key(&high_key) {
                let high = *answers.get(&high_key).unwrap();
                if check_answer <= high {
                    return format!("Given value ({}) is less than current high ({})", check_answer, high);
                }
            }
        }
        return String::new();
    }
}

pub struct AssumeWords<R: BufRead> {
    inner: AssumeChars<R>,
    sep: &'static str,
    word: String,
}

impl<R: BufRead> Iterator for AssumeWords<R> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            while !self.word.ends_with(self.sep) && let Some(c) = self.inner.next() {
                self.word.push(c);
            }
            if self.word.ends_with(self.sep) {
                self.word = self.word.trim_end_matches(self.sep).to_string();
                if self.word.len() == 0 {
                    continue;
                }
            }
            if self.word.len() == 0 {
                return None;
            }
            else {
                return Some(std::mem::take(&mut self.word));
            }
        }
    }
}

pub struct AssumeLines<R: BufRead> {
    inner: Lines<R>,
}

impl<R: BufRead> Iterator for AssumeLines<R> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|l| l.unwrap())
    }
}

trait AsssumeBufRead: BufRead + Sized {
    fn assume_lines(self) -> AssumeLines<Self> {
        AssumeLines { inner: self.lines() }
    }
    fn assume_chars(self) -> AssumeChars<Self> {
        AssumeChars { inner: self.bytes() }
    }
    fn assume_words(self, sep: &'static str) -> AssumeWords<Self> {
        AssumeWords { inner: self.assume_chars(), sep: sep, word: String::new() }
    }
}

impl<R: BufRead> AsssumeBufRead for R {}

pub struct AssumeChars<R: BufRead> {
    inner: Bytes<R>
}

impl<R: BufRead> Iterator for AssumeChars<R> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|b| b.unwrap() as char)
    }
}

pub mod log {
    static LOG_LEVEL : std::sync::OnceLock<LogLevel> = std::sync::OnceLock::new();
    pub fn parse_and_set_log_level(level_string: &str) {
        let level;
        if level_string.len() > 0 {
            level = level_string.parse().expect("Failed to parse log level!");
        } else {
            level = LogLevel::Error;
        }

        // If this gets annoying, just ignore the Err
        LOG_LEVEL.set(level).expect("Log level set twice!");
    }

    pub fn error<F: FnOnce() -> String>(msg_fn: F) {
        log(LogLevel::Error, msg_fn);
    }
    pub fn warn<F: FnOnce() -> String>(msg_fn: F) {
        log(LogLevel::Warning, msg_fn);
    }
    pub fn info<F: FnOnce() -> String>(msg_fn: F) {
        log(LogLevel::Info, msg_fn);
    }
    pub fn debug<F: FnOnce() -> String>(msg_fn: F) {
        log(LogLevel::Debug, msg_fn);
    }
    fn log<F: FnOnce() -> String>(level: LogLevel, msg_fn: F) {
        if level <= *LOG_LEVEL.get_or_init(|| LogLevel::Error) {
            let prefix = match level {
                LogLevel::Error => "ERROR",
                LogLevel::Warning => "WARN",
                LogLevel::Info => "INFO",
                LogLevel::Debug => "DEBUG",
                LogLevel::None => "",
            };
            let msg = msg_fn();
            eprintln!("[{prefix}] {msg}");
        }
    }


    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    enum LogLevel {
        None,
        Error,
        Warning,
        Info,
        Debug,
    }

    impl std::str::FromStr for LogLevel {
        type Err = ParseLogLevelError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "none" => Ok(LogLevel::None),
                "error" | "err" => Ok(LogLevel::Error),
                "warn" | "warning" => Ok(LogLevel::Warning),
                "info" => Ok(LogLevel::Info),
                "debug" => Ok(LogLevel::Debug),
                _ => Err(ParseLogLevelError("Could not match input string".to_string()))
            }
        }
    }

    #[derive(Debug)]
    struct ParseLogLevelError(String);
    impl std::error::Error for ParseLogLevelError {}
    impl std::fmt::Display for ParseLogLevelError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LogLevel parse error: {}", self.0)
        }
    }
}

pub trait StrNum {
    fn num_cmp(self: Self, other: Self) -> cmp::Ordering;
    fn num_gt(self: Self, other: Self) -> bool;
    fn num_ge(self: Self, other: Self) -> bool;
    fn num_lt(self: Self, other: Self) -> bool;
    fn num_le(self: Self, other: Self) -> bool;
    fn num_eq(self: Self, other: Self) -> bool;
}

impl StrNum for &str {
    fn num_cmp(self: Self, other: Self) -> cmp::Ordering {
        if self.len() > other.len() {
            return cmp::Ordering::Greater;
        }
        else if self.len() < other.len() {
            return cmp::Ordering::Less;
        }
        else {
            return self.cmp(other);
        }
    }
    fn num_gt(self: Self, other: Self) -> bool {
        return self.num_cmp(other) == cmp::Ordering::Greater;
    }
    fn num_ge(self: Self, other: Self) -> bool {
        return self.num_cmp(other) != cmp::Ordering::Less;
    }
    fn num_lt(self: Self, other: &str) -> bool {
        return self.num_cmp(other) == cmp::Ordering::Less;
    }
    fn num_le(self: Self, other: Self) -> bool {
        return self.num_cmp(other) != cmp::Ordering::Greater;
    }
    fn num_eq(self: Self, other: Self) -> bool {
        return self.num_cmp(other) == cmp::Ordering::Equal;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_num_cmp() {
        let a = vec![
            "95",
            "123",
            "222222",
            "899845",
            "987654322",
            "987654322",
            "98",
        ];
        let b = vec![
            "115",
            "123",
            "222122",
            "999845",
            "987654321",
            "987654",
            "987654",
        ];
        let res = vec![
            cmp::Ordering::Less,
            cmp::Ordering::Equal,
            cmp::Ordering::Greater,
            cmp::Ordering::Less,
            cmp::Ordering::Greater,
            cmp::Ordering::Greater,
            cmp::Ordering::Less,
        ];
        for i in 0..a.len() {
            assert_eq!(a[i].num_cmp(b[i]), res[i], "Failed at index on trait \"{i}\"");
        }
    }
}
