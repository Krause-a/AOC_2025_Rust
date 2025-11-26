#![allow(unused, dead_code)]

use std::{fs, io::{self, BufRead, Read}};

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
    pub fn get_lines(self: &Self) -> Result<io::Lines<io::BufReader<fs::File>>, io::Error> {
        let f = self.open_file()?;
        let reader = io::BufReader::new(f);
        Ok(reader.lines())
    }
    pub fn get_chars(self: &Self) -> Result<impl Iterator<Item = io::Result<char>>, io::Error> {
        let f = self.open_file()?;
        let reader = io::BufReader::new(f);
        Ok(reader.bytes().map(|br| br.map(|b| b as char)))
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

    pub fn error(msg: &str) {
        log(LogLevel::Error, msg);
    }
    pub fn warn(msg: &str) {
        log(LogLevel::Warning, msg);
    }
    pub fn info(msg: &str) {
        log(LogLevel::Info, msg);
    }
    pub fn debug(msg: &str) {
        log(LogLevel::Debug, msg);
    }
    fn log(level: LogLevel, msg: &str) {
        if level <= *LOG_LEVEL.get_or_init(|| LogLevel::Error) {
            let prefix = match level {
                LogLevel::Error => "ERROR",
                LogLevel::Warning => "WARN",
                LogLevel::Info => "INFO",
                LogLevel::Debug => "DEBUG",
                LogLevel::None => "",
            };
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
