use std::mem::swap;

pub trait Tuple<T>: Clone + Sized {
	fn collect<I: Iterator<Item = T>>(it: I) -> Option<Self>;
	fn push_right(&mut self, v: T);
}

impl<T: Clone> Tuple<T> for (T, T) {
	fn collect<I: Iterator<Item = T>>(mut it: I) -> Option<Self> {
		Some((it.next()?, it.next()?))
	}

	fn push_right(&mut self, v: T) {
		swap(&mut self.0, &mut self.1);
		self.1 = v;
	}
}

impl<T: Clone> Tuple<T> for (T, T, T) {
	fn collect<I: Iterator<Item = T>>(mut it: I) -> Option<Self> {
		Some((it.next()?, it.next()?, it.next()?))
	}

	fn push_right(&mut self, v: T) {
		swap(&mut self.0, &mut self.1);
		swap(&mut self.1, &mut self.2);
		self.2 = v;
	}
}

impl<T: Clone> Tuple<T> for (T, T, T, T) {
	fn collect<I: Iterator<Item = T>>(mut it: I) -> Option<Self> {
		Some((it.next()?, it.next()?, it.next()?, it.next()?))
	}

	fn push_right(&mut self, v: T) {
		swap(&mut self.0, &mut self.1);
		swap(&mut self.1, &mut self.2);
		swap(&mut self.2, &mut self.3);
		self.3 = v;
	}
}

impl<T: Clone> Tuple<T> for [T; 2] {
	fn collect<I: Iterator<Item = T>>(mut it: I) -> Option<Self> {
		Some([it.next()?, it.next()?])
	}

	fn push_right(&mut self, v: T) {
		self.swap(0, 1);
		self[1] = v;
	}
}

pub struct TupleWindows<I, T> {
	pub(crate) iter: I,
	pub(crate) tuple: Option<T>,
}

impl<I, T, U> Iterator for TupleWindows<I, T>
where
	I: Iterator<Item = U>,
	T: Tuple<U>,
	U: Copy,
{
	type Item = T;

	fn next(&mut self) -> Option<T> {
		let last = self.tuple.as_mut()?;
		last.push_right(self.iter.next()?);
		Some(last.clone())
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, None)
	}
}
