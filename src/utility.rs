#![allow(unused, dead_code)]

use std::{fs, io::{self, BufRead, Bytes, Lines, Read}};

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
