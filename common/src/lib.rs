use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Not;
use std::str::FromStr;

pub fn read_all_lines(day: &str) -> impl Iterator<Item = String> {
	BufReader::new(File::open(format!("input/day{}.txt", day)).unwrap())
		.lines()
		.map(|l| l.unwrap())
}

pub fn read_lines(day: &str) -> impl Iterator<Item = String> {
	read_all_lines(day).filter(|l| !l.is_empty())
}

pub fn try_read_lines_as<T: FromStr>(day: &str) -> impl Iterator<Item = Result<T, String>> {
	read_lines(day).map(|l| T::from_str(&l).map_err(|_| l))
}

pub fn read_lines_as<T: FromStr>(day: &str) -> impl Iterator<Item = T> {
	try_read_lines_as(day).map(|l| l.unwrap())
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
			if let v @ Some(_) = f(&mut state, item) {
				return v;
			}
		}
		None
	}

	fn next_doublet(&mut self) -> Option<(Self::Item, Self::Item)> {
		Some((self.next()?, self.next()?))
	}

	fn next_doublet_array(&mut self) -> Option<[Self::Item; 2]> {
		Some([self.next()?, self.next()?])
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

pub struct Bitmap<const N: usize> {
	inner: [u64; N],
}

fn mask(i: usize) -> u64 {
	1 << (i & 0x3F)
}

impl<const N: usize> Bitmap<N> {
	pub fn new() -> Self {
		Self { inner: [0; N] }
	}

	fn set_bit(&mut self, i: usize, f: bool) {
		assert!(i < N * 64);
		unsafe {
			if f {
				*self.inner.get_unchecked_mut(i >> 6) |= mask(i);
			} else {
				*self.inner.get_unchecked_mut(i >> 6) &= !mask(i);
			}
		}
	}

	pub fn set(&mut self, i: usize) {
		self.set_bit(i, true)
	}

	pub fn unset(&mut self, i: usize) {
		self.set_bit(i, false)
	}

	pub fn toggle(&mut self, i: usize) {
		if self.get(i) {
			self.unset(i)
		} else {
			self.set(i)
		}
	}

	pub fn inverse(&mut self) {
		for x in &mut self.inner {
			*x = !*x;
		}
	}

	pub fn get(&self, i: usize) -> bool {
		assert!(i < N * 64);
		self.inner[i >> 6] & mask(i) > 0
	}

	pub fn cardinality(&self) -> u32 {
		self.inner.iter().map(|i| i.count_ones()).sum()
	}
}

impl<const N: usize> Not for Bitmap<N> {
	type Output = Self;

	fn not(mut self) -> Self {
		self.inverse();
		self
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
