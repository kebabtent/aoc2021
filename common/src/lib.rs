pub use self::bitmap::*;
pub use self::tuple::*;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;
use std::str::FromStr;

mod bitmap;
mod tuple;

pub fn read_all_lines() -> impl Iterator<Item = String> {
	BufReader::new(
		File::open(format!(
			"input/{}.txt",
			std::env::var("CARGO_PKG_NAME").unwrap()
		))
		.unwrap(),
	)
	.lines()
	.filter_map(|l| l.ok())
}

pub fn read_lines() -> impl Iterator<Item = String> {
	read_all_lines().filter(|l| !l.is_empty())
}

pub fn try_read_lines_as<T: FromStr>() -> impl Iterator<Item = Result<T, String>> {
	read_lines().map(|l| T::from_str(&l).map_err(|_| l))
}

pub fn read_lines_as<T: FromStr>() -> impl Iterator<Item = T> {
	try_read_lines_as().filter_map(|l| l.ok())
}

pub trait Add<Rhs = Self> {
	type Output;
	fn add(self, rhs: Rhs) -> Self::Output;
}

impl<T> Add<T> for T
where
	T: std::ops::Add<T>,
{
	type Output = T::Output;
	fn add(self, rhs: T) -> Self::Output {
		self + rhs
	}
}

macro_rules! impl_bool_add {
	( $( $x:ty ),* ) => {
		$(
			impl Add<bool> for $x {
				type Output = $x;
				fn add(self, rhs: bool) -> Self::Output {
					self + rhs as $x
				}
			}
		)*
	};
}

impl_bool_add!(u32, u64, usize);

pub trait DoubletSum<A, B>: Iterator<Item = (A, B)> + Sized {
	fn doublet_sum<SA, SB>(self) -> (SA, SB)
	where
		SA: Add<A, Output = SA> + Default,
		SB: Add<B, Output = SB> + Default,
	{
		self.fold((SA::default(), SB::default()), |(a, b), (c, d)| {
			(a.add(c), b.add(d))
		})
	}
}

impl<T, A, B> DoubletSum<A, B> for T where T: Iterator<Item = (A, B)> {}

pub trait IterExt: Iterator + Sized {
	fn batching<B, F>(self, f: F) -> Batching<Self, F>
	where
		F: FnMut(&mut Self) -> Option<B>,
	{
		Batching { f, iter: self }
	}

	fn fold_while<S, B, F>(&mut self, mut state: S, mut f: F) -> Option<B>
	where
		Self: Sized,
		F: FnMut(&mut S, Self::Item) -> Option<B>,
	{
		while let Some(item) = self.next() {
			let v = f(&mut state, item);
			if v.is_some() {
				return v;
			}
		}
		None
	}

	fn next_tuple<T: Tuple<Self::Item>>(&mut self) -> Option<T> {
		Tuple::collect(self)
	}

	fn tuple_windows<T>(mut self) -> TupleWindows<Self, T>
	where
		T: Tuple<Self::Item>,
		Self::Item: Clone,
	{
		let tuple = self
			.next()
			.and_then(|f| T::collect(once(f.clone()).chain(once(f)).chain(&mut self)));

		TupleWindows { iter: self, tuple }
	}
}

impl<T> IterExt for T where T: Iterator {}

pub struct Batching<I, F> {
	f: F,
	iter: I,
}

impl<B, F, I> Iterator for Batching<I, F>
where
	I: Iterator,
	F: FnMut(&mut I) -> Option<B>,
{
	type Item = B;
	fn next(&mut self) -> Option<B> {
		(self.f)(&mut self.iter)
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, None)
	}
}

pub struct Buffer<T> {
	capacity: usize,
	inner: VecDeque<T>,
}

impl<T> Buffer<T> {
	pub fn new(capacity: usize) -> Self {
		Self {
			capacity,
			inner: VecDeque::with_capacity(capacity),
		}
	}

	pub fn capacity(&self) -> usize {
		self.capacity
	}

	pub fn len(&self) -> usize {
		self.inner.len()
	}

	pub fn is_full(&self) -> bool {
		self.capacity == self.inner.len()
	}

	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.inner.iter()
	}

	pub fn push(&mut self, value: T) -> Option<T> {
		let res = if self.is_full() {
			self.inner.pop_front()
		} else {
			None
		};
		self.inner.push_back(value);
		res
	}
}
