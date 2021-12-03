use std::ops::Not;

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
