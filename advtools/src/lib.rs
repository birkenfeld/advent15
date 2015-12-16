use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::marker::PhantomData;
use std::ops::Add;

pub trait Input {
    fn consumes(line: &str) -> usize { line.len() }
    fn read_token(tok: &str) -> Self;
}

impl Input for char {
    fn consumes(line: &str) -> usize {
        let mut iter = line.char_indices();
        match iter.next() {
            None => 0,
            Some(_) => match iter.next() {
                None => line.len(),
                Some((idx, _)) => idx
            }
        }
    }
    fn read_token(tok: &str) -> char {
        tok.chars().next().unwrap()
    }
}

impl Input for String {
    fn read_token(tok: &str) -> String {
        tok.into()
    }
}

impl Input for Vec<String> {
    fn read_token(tok: &str) -> Vec<String> {
        tok.split_whitespace().map(String::from).collect()
    }
}

macro_rules! simple_impl {
    ($ty:ty) => {
        impl Input for $ty {
            fn read_token(tok: &str) -> $ty {
                tok.parse().unwrap()
            }
        }
    }
}

simple_impl!(u8);
simple_impl!(u16);
simple_impl!(u32);
simple_impl!(u64);
simple_impl!(i8);
simple_impl!(i16);
simple_impl!(i32);
simple_impl!(i64);

impl<T: Input> Input for (T,) {
    fn read_token(tok: &str) -> (T,) {
        (T::read_token(tok),)
    }
}

macro_rules! tuple_impl {
    ($($tys:ident),+) => {
        impl<$($tys: Input),+> Input for ($($tys),+) {
            fn read_token(tok: &str) -> ($($tys),+) {
                let mut toks = tok.split_whitespace();
                ($($tys::read_token(toks.next().unwrap())),+)
            }
        }
    }
}

tuple_impl!(T, U);
tuple_impl!(T, U, V);
tuple_impl!(T, U, V, W);
tuple_impl!(T, U, V, W, Y);
tuple_impl!(T, U, V, W, Y, Z);
tuple_impl!(T, U, V, W, Y, Z, T1);
tuple_impl!(T, U, V, W, Y, Z, T1, T2);
tuple_impl!(T, U, V, W, Y, Z, T1, T2, T3);
tuple_impl!(T, U, V, W, Y, Z, T1, T2, T3, T4);
tuple_impl!(T, U, V, W, Y, Z, T1, T2, T3, T4, T5);


pub struct InputIterator<I: Input> {
    rdr: BufReader<File>,
    marker: PhantomData<I>,
    line: String,
}

impl<I: Input> Iterator for InputIterator<I> {
    type Item = I;

    fn next(&mut self) -> Option<I> {
        while self.line.is_empty() {
            if self.rdr.read_line(&mut self.line).unwrap() == 0 {
                return None;
            }
            while self.line.trim_right() != self.line {
                self.line.pop();
            }
        }
        let n = I::consumes(&self.line);
        let res = I::read_token(&self.line[..n]);
        self.line = self.line[n..].into();
        Some(res)
    }
}


pub fn iter_input<I: Input>() -> InputIterator<I> {
    let fp = File::open("input.txt").expect("input file \"input.txt\" not found in cwd");
    let rdr = BufReader::new(fp);
    InputIterator { rdr: rdr, marker: PhantomData, line: String::new() }
}


pub trait IterExt: Iterator {
    fn sum_from<S=<Self as Iterator>::Item>(self, start: S) -> S where
        S: Add<Self::Item, Output=S>, Self: Sized,
    {
        self.fold(start, |s, e| s + e)
    }
}

impl<I: Iterator> IterExt for I { }
