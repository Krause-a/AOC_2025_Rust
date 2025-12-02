#![allow(unused, dead_code)]

use std::{cmp, fs, io::{self, BufRead, Bytes, Lines, Read}};

pub struct TestData {
    file_path: std::path::PathBuf,
}

impl TestData {
    pub fn new(file_path: std::path::PathBuf) -> TestData {
        TestData { file_path: file_path }
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

#[deprecated = "Just use <,>,="]
pub fn str_num_compare(a: &str, b: &str) -> cmp::Ordering {
    if a.len() != b.len() {
        if a.len() > b.len() {
            return cmp::Ordering::Greater;
        }
        else {
            return cmp::Ordering::Less;
        }
    }
    else {
        for i in 0..a.len() {
            let a_digit = a[i..=i].parse::<u8>().unwrap();
            let b_digit = b[i..=i].parse::<u8>().unwrap();
            if a_digit > b_digit {
                return cmp::Ordering::Greater;
            }
            else if a_digit < b_digit {
                return cmp::Ordering::Less;
            }
        }
    }
    return cmp::Ordering::Equal;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_num_compare() {
        let a = vec![
            "123",
            "222222",
            "899845",
            "987654322",
            "987654322",
            "98",
        ];
        let b = vec![
            "123",
            "222122",
            "999845",
            "987654321",
            "987654",
            "987654",
        ];
        let res = vec![
            cmp::Ordering::Equal,
            cmp::Ordering::Greater,
            cmp::Ordering::Less,
            cmp::Ordering::Greater,
            cmp::Ordering::Greater,
            cmp::Ordering::Less,
        ];
        for i in 0..a.len() {
            assert_eq!(str_num_compare(a[i], b[i]), res[i], "Failed at index \"{i}\"");
            assert_eq!(a[i].cmp(b[i]), res[i], "Failed at index on cmp \"{i}\"");
        }
    }
}
