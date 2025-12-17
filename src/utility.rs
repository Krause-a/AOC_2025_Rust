#![allow(unused, dead_code)]

use std::{cmp, collections::HashMap, fs, hash::{BuildHasher, Hash, Hasher}, io::{self, BufRead, Bytes, Lines, Read}, str::FromStr};

pub struct TestData {
    file_path: std::path::PathBuf,
    day: usize,
    part: usize,
    test: bool,
}

impl TestData {
    pub fn new(file_path: std::path::PathBuf, day: usize, part: usize, test: bool) -> TestData {
        let mut test_data = TestData { file_path: file_path, day: day, part: part, test: test };
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
    pub fn get_grid(self: &Self) -> Result<HashMap<Point, char>, io::Error> {
        let f = self.open_file()?;
        let reader = io::BufReader::new(f);
        let mut map = HashMap::new();

        for (y, line) in reader.lines().enumerate() {
            let line = line?;
            for (x, c) in line.chars().enumerate() {
                map.insert(Point::new(x as isize, y as isize), c);
            }
        }
        Ok(map)
    }
    pub fn is_test(self: &Self) -> bool {
        return self.test;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
    pub fn cardinal_neighbors(self: &Self) -> [Point; 4] {
        [
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y - 1),
        ]
    }
    pub fn diagonal_neighbors(self: &Self) -> [Point; 4] {
        [
            Point::new(self.x + 1, self.y + 1),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x + 1, self.y - 1),
        ]
    }
    pub fn all_neighbors(self: &Self) -> [Point; 8] {
        [
            Point::new(self.x + 1, self.y),
            Point::new(self.x + 1, self.y + 1),
            Point::new(self.x, self.y + 1),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y - 1),
        ]
    }
    pub fn cardinal_neighbors_in<T>(self: &Self, map: &HashMap<Point, T, impl BuildHasher>) -> Vec<Point> {
        self.cardinal_neighbors().into_iter().filter(|p| map.contains_key(&p)).collect()
    }
    pub fn diagonal_neighbors_in<T>(self: &Self, map: &HashMap<Point, T, impl BuildHasher>) -> Vec<Point> {
        self.diagonal_neighbors().into_iter().filter(|p| map.contains_key(&p)).collect()
    }
    pub fn all_neighbors_in<T>(self: &Self, map: &HashMap<Point, T, impl BuildHasher>) -> Vec<Point> {
        self.all_neighbors().into_iter().filter(|p| map.contains_key(&p)).collect()
    }
    pub fn add(self: &Self, other: &Self) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
    pub fn sub(self: &Self, other: &Self) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl FromStr for Point {
    type Err = ParsePointError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = vec!['-','1','2','3','4','5','6','7','8','9','0'];
        let splits = s.split(|c| !nums.contains(&c));
        let mut v1 = None;
        let mut v2 = None;
        for split in splits {
            if split.len() < 1 {
                continue;
            }
            if v1.is_none() {
                v1 = Some(split)
            } else if v2.is_none() {
                v2 = Some(split)
            } else {
                return Err(ParsePointError("Too many items in string".to_string()));
            }
        }
        if v1.is_some() && v2.is_some() {
            let v1_num = v1.unwrap().parse().map_err(|_| ParsePointError("Item is not an isize".to_string()))?;
            let v2_num = v2.unwrap().parse().map_err(|_| ParsePointError("Item is not an isize".to_string()))?;
            Ok(Point::new(v1_num, v2_num))
        }
        else {
            Err(ParsePointError("Not enough items in string".to_string()))
        }
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_isize(self.x);
        state.write_isize(self.y);
    }
}

#[derive(Debug)]
pub struct ParsePointError(String);
impl std::error::Error for ParsePointError {}
impl std::fmt::Display for ParsePointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point parse error: {}", self.0)
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
    use std::hash::BuildHasher;

    use super::Point;
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

    // For now I am assuming that any inline type of logging would be debug
    pub fn inline<T, F: FnOnce() -> String>(t: T, msg_fn: F) -> T {
        debug(msg_fn);
        t
    }

    // For now I am assuming that any grid type of logging would be debug
    pub fn grid<T>(grid: &std::collections::HashMap<Point, T, impl BuildHasher>)
        where T: std::fmt::Display,
    {
        debug(|| {
            let mut max_p : Option<Point> = None;
            let mut min_p : Option<Point> = None;
            for (p, c) in grid.iter() {
                if let Some(max) = max_p.as_mut() {
                    max.x = max.x.max(p.x);
                    max.y = max.y.max(p.y);
                }
                else {
                    max_p = Some(p.to_owned());
                }
                if let Some(min) = min_p.as_mut() {
                    min.x = min.x.min(p.x);
                    min.y = min.y.min(p.y);
                }
                else{
                    min_p = Some(p.to_owned());
                }
            }
            let Some(min_p) = min_p else {return String::new()};
            let Some(max_p) = max_p else {return String::new()};
            let mut all_lines : Vec<String> = Vec::with_capacity((max_p.y - min_p.y) as usize);
            let mut line = String::with_capacity((max_p.x - min_p.x) as usize);
            for y in min_p.y..=max_p.y {
                line.clear();
                for x in min_p.x..=max_p.x {
                    let p = Point::new(x, y);
                    if let Some(grid_value) = grid.get(&p) {
                        line.push_str(&format!("{}", grid_value));
                    }
                    else {
                        line.push(' ');
                    }
                }
                all_lines.push(line.clone());
            }
            "\n".to_string() + &all_lines.join("\n")
        });

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
    fn is_num(self: &Self) -> bool;
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
    fn is_num(self: &Self) -> bool {
        self.chars().all(|c| c.is_digit(10))
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

#[derive(Default, Clone)]
pub struct FastHasher {
    hash: u64,
}

impl FastHasher {
    #[inline]
    fn mix(hash: u64, x: u64) -> u64 {
        let mut z = hash ^ x;
        z = z.wrapping_add(0x9e3779b97f4a7c15);
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

impl Hasher for FastHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.hash = Self::mix(self.hash, (i as u64));
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let mut i = 0;
        while i + 8 <= bytes.len() {
            let chunk = u64::from_be_bytes(bytes[i..i+8].try_into().expect("Failed bytes conversion"));
            self.hash = Self::mix(self.hash, chunk);
            i += 8;
        }
        if i < bytes.len() {
            let mut tail = 0u64;
            for (shift, &b) in bytes[i..].iter().enumerate() {
                tail |= (b as u64) << (shift * 8);
            }
            self.hash = Self::mix(self.hash, tail);
        }
    }
}

#[derive(Default, Clone)]
pub struct FastBuildHasher;

impl BuildHasher for FastBuildHasher {
    type Hasher = FastHasher;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        FastHasher::default()
    }
}
