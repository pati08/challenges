use std::io::Read;
use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct DefaultArgs {
    #[arg(short, long, value_name = "PATH")]
    #[cfg(test)]
    pub input_file: String,

    #[arg(short, long, value_name = "PATH")]
    #[cfg(not(test))]
    pub input_file: String,
}
impl DefaultArgs {
    pub fn get_input(&self, trim: bool) -> Input {
        if self.input_file.trim() == "-" {
            get_stdin_input(trim)
        } else {
            get_file_input(self.input_file.clone().into(), trim)
        }
    }
}
pub struct DefaultProblemDesc;
impl ProblemDesc for DefaultProblemDesc {
    type Args = DefaultArgs;
    const TRIM: bool = true;
}

pub trait ProblemDesc {
    type Args: clap::Parser;
    const TRIM: bool = true;
}

/// Datastructure to egonomically read input from stdin or a file.
#[derive(Clone)]
pub struct Input {
    original: String,
    ptr: usize,
    lines: Vec<String>,
}
impl Input {
    /// Construct from a list of strings
    pub fn new(text: String, trim: bool) -> Self {
        let lines = text
            .lines()
            .map(|line| {
                if trim {
                    line.trim().to_string()
                } else {
                    line.to_string()
                }
            })
            .collect();
        Self {
            ptr: 0,
            lines,
            original: text,
        }
    }
    /// Get the original text
    pub fn get_original(&self) -> &str {
        &self.original
    }
    /// Get the next line as a string
    ///
    /// # Panics
    /// Panics if the next line is out of bounds
    pub fn next_line(&mut self) -> Option<String> {
        let res = self.lines.get(self.ptr).cloned();
        self.ptr += 1;
        res
    }
    #[allow(clippy::should_implement_trait)]
    /// Get the next line parsed with `FromStr` as type `T`
    ///
    /// # Panics
    /// Panics if the next line is out of bounds or `FromStr::parse` fails
    pub fn next<T>(&mut self) -> Option<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        if self.ptr >= self.lines.len() {
            return None;
        }
        let res = self.next_line()?.parse();
        if res.is_err() {
            self.ptr -= 1;
        }
        res.ok()
    }
    /// Skip `n` lines
    pub fn skip(&mut self, n: usize) {
        self.ptr += n;
    }
    /// Skips the next `n` lines and returns an iterator over the skipped lines
    ///
    /// Note that the lines are still skipped even if the iterator is not consumed
    pub fn take(&mut self, n: usize) -> impl Iterator<Item = &str> {
        let start = self.ptr;
        self.ptr += n;
        self.lines[start..self.ptr].iter().map(String::as_str)
    }
    /// Construct from a reader
    pub fn from_reader<R: std::io::Read>(reader: R, trim: bool) -> Self {
        let mut reader = std::io::BufReader::new(reader);
        let mut text = String::new();
        reader.read_to_string(&mut text).unwrap();
        Self::new(text, trim)
    }
    pub fn lines(&self) -> InputLines {
        InputLines {
            ptr: self.ptr,
            lines: self.lines.clone(),
        }
    }
    pub fn lines_consuming<'a>(&'a mut self) -> InputLinesConsuming<'a> {
        InputLinesConsuming { input: self }
    }
}

pub struct InputLinesConsuming<'a> {
    input: &'a mut Input,
}
impl Iterator for InputLinesConsuming<'_> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.input.next_line()
    }
}

pub struct InputLines {
    ptr: usize,
    lines: Vec<String>,
}

impl Iterator for InputLines {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr >= self.lines.len() {
            return None;
        }
        let line = self.lines[self.ptr].clone();
        self.ptr += 1;
        Some(line)
    }
}

fn get_file_input(filepath: PathBuf, trim: bool) -> Input {
    if !filepath.is_file() {
        panic!("Input file does not exist.");
    }
    let raw_input = std::fs::read_to_string(filepath).expect("Failed to read input file.");
    Input::new(raw_input, trim)
}

fn get_stdin_input(trim: bool) -> Input {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    Input::from_reader(reader, trim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_take() {
        let mut input = Input::new("1\nhello\nworld\nrust".to_string(), false);
        let _ = input.next::<String>();
        let _ = input.take(2);
        assert_eq!(input.ptr, 3);
    }

    #[test]
    fn input_next() {
        let mut input = Input::new("1\nhello\nworld\nrust".to_string(), false);
        let num: i32 = input.next().unwrap();
        assert_eq!(num, 1);
        assert_eq!(input.ptr, 1);
        let mystr: String = input.next().unwrap();
        assert_eq!(mystr, "hello");
        assert_eq!(input.ptr, 2);
    }

    #[test]
    fn input_next_line() {
        let mut input = Input::new("hello\nworld\nrust".to_string(), false);
        let line = input.next_line().unwrap();
        assert_eq!(line, "hello");
        assert_eq!(input.ptr, 1);
    }
}
