#![allow(unused_imports, unused_macros)]

use std::array;
use std::fmt::Display;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::str::FromStr;

pub use crate::misc::macros::with_dollar_sign;

pub struct FastIO<I: Read, O: Write> {
    token_buf: Vec<String>,
    reader: BufReader<I>,
    writer: BufWriter<O>,
}

impl<I: Read, O: Write> FastIO<I, O> {
    pub fn new(reader: I, writer: O) -> Self {
        Self {
            token_buf: Vec::new(),
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
        }
    }

    pub fn read<T: FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.token_buf.pop() {
                return token.parse().ok().expect("Failed to parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed to read");
            self.token_buf =
                input.split_whitespace().rev().map(String::from).collect();
        }
    }

    pub fn read_array<T: FromStr, const N: usize>(&mut self) -> [T; N] {
        array::from_fn(|_| self.read())
    }

    pub fn read_vec<T: FromStr>(&mut self, count: usize) -> Vec<T> {
        (0..count).map(|_| self.read()).collect()
    }

    pub fn read_grid<T: FromStr>(
        &mut self,
        rows: usize,
        cols: usize,
    ) -> Vec<Vec<T>> {
        (0..rows).map(|_| self.read_vec(cols)).collect()
    }

    pub fn read_line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed to read");
        if input.ends_with('\n') {
            input.pop();
        }
        input
    }

    pub fn write(&mut self, out: &str) {
        self.writer.write(out.as_bytes()).expect("Failed to write");
    }

    pub fn write_line(&mut self, out: &str) {
        self.write(out);
        self.write("\n");
    }
}

#[macro_export]
macro_rules! with_fast_io {
    ($name:ident, $input:ident, $output:ident) => {
        #[allow(unused_mut)]
        let mut $name = FastIO::new($input, $output);

        with_dollar_sign! {
            ($d:tt) => {
                macro_rules! cout_fmt {
                    ($d fmt:expr, $d($d arg:expr),*) => {
                        $name.write(&format!($d fmt, $d($d arg),*));
                    }
                }
                macro_rules! coutln_fmt {
                    ($d fmt:expr, $d($d arg:expr),*) => {
                        $name.write_line(&format!($d fmt, $d($d arg),*));
                    }
                }
                macro_rules! to_format {
                    ($d cur:expr) => {
                        "{}"
                    };
                    ($d cur:expr, $d($d rest:expr),*) => {
                        concat!(to_format!($d cur), to_format!($d($d rest),*))
                    };
                }
                macro_rules! cout {
                    ($d($d arg:expr),*) => {
                        cout_fmt!(to_format!($d($d arg),*), $d($d arg),*);
                    }
                }
                macro_rules! coutln {
                    ($d($d arg:expr),*) => {
                        coutln_fmt!(to_format!($d($d arg),*), $d($d arg),*);
                    }
                }
            }
        }
    }
}

pub use with_fast_io;

#[cfg(test)]
mod test {
    use std::io::BufReader;
    use std::io::{stdin, stdout};

    use super::*;

    #[test]
    fn test_io() {
        let input = BufReader::new(
            concat!(
                "69 420\n",
                "this entire line\n",
                "420.69 string\n",
                "3 4\n",
                "a b c d\n",
                "e f g h\n",
                "i j k l\n",
            )
            .as_bytes(),
        );
        let output = Vec::new();

        with_fast_io!(io, input, output);

        let [a, b] = io.read_array::<i32, 2>();
        assert_eq!(a, 69);
        assert_eq!(b, 420);

        let line = io.read_line();
        assert_eq!(line, "this entire line");

        let float: f64 = io.read();
        assert_eq!(float, 420.69);

        let word: String = io.read();
        assert_eq!(word, "string");

        let [n, m] = io.read_array::<usize, 2>();
        let grid = io.read_grid::<String>(n, m);
        assert_eq!(
            grid,
            [
                ["a", "b", "c", "d"],
                ["e", "f", "g", "h"],
                ["i", "j", "k", "l"],
            ]
        );

        cout!("test: ");
        coutln!("newline here");
        coutln_fmt!("word: {}", word);
        coutln_fmt!("float: {:.04}", float);

        let vec = io.writer.into_inner().unwrap();
        assert_eq!(vec, b"test: newline here\nword: string\nfloat: 420.6900\n");
    }
}
