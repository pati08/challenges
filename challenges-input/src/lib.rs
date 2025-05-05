use std::io::BufRead;
use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct DefaultArgs {
    #[arg(short, long, value_name = "PATH")]
    input_file: String,
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
pub struct Input {
    ptr: usize,
    lines: Vec<String>,
}
impl Input {
    /// Construct from a list of strings
    pub fn new(lines: Vec<String>) -> Self {
        Self { ptr: 0, lines }
    }
    /// Get the next line as a string
    ///
    /// # Panics
    /// Panics if the next line is out of bounds
    pub fn next_line(&mut self) -> String {
        self.ptr += 1;
        self.lines[self.ptr - 1].clone()
    }
    #[allow(clippy::should_implement_trait)]
    /// Get the next line parsed with `FromStr` as type `T`
    ///
    /// # Panics
    /// Panics if the next line is out of bounds or `FromStr::parse` fails
    pub fn next<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        self.next_line().parse().unwrap()
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
        let reader = std::io::BufReader::new(reader);
        let lines = reader.lines().map_while(Result::ok);
        let lines = if trim {
            lines.map(|v| v.trim().to_string()).collect()
        } else {
            lines.collect()
        };
        Self { ptr: 0, lines }
    }
}

fn get_file_input(filepath: PathBuf, trim: bool) -> Input {
    if !filepath.is_file() {
        panic!("Input file does not exist.");
    }
    let raw_input = std::fs::read_to_string(filepath).expect("Failed to read input file.");
    let input = raw_input
        .lines()
        .map(|line| {
            if trim {
                line.trim().to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>();
    Input::new(input)
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
        let mut input = Input::new(vec![
            "1".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "rust".to_string(),
        ]);
        let _ = input.next::<String>();
        let _ = input.take(2);
        assert_eq!(input.ptr, 3);
    }

    #[test]
    fn input_next() {
        let mut input = Input::new(vec![
            "1".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "rust".to_string(),
        ]);
        let num: i32 = input.next();
        assert_eq!(num, 1);
        assert_eq!(input.ptr, 1);
        let mystr: String = input.next();
        assert_eq!(mystr, "hello");
        assert_eq!(input.ptr, 2);
    }

    #[test]
    fn input_next_line() {
        let mut input = Input::new(vec![
            "hello".to_string(),
            "world".to_string(),
            "rust".to_string(),
        ]);
        let line = input.next_line();
        assert_eq!(line, "hello");
        assert_eq!(input.ptr, 1);
    }
}
