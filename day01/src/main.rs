use common::{read_lines_as, DoubletSum};
use itertools::Itertools;

fn main() {
	let (a, b) = read_lines_as::<u32>()
		.tuple_windows()
		.enumerate()
		.map(|(i, (a, b, c, d))| (i < 2 && b > a, d > c, b + c + d > a + b + c))
		.map(|(a, b, c)| (a as u32 + b as u32, c))
		.doublet_sum::<u32, u32>();

	println!("{}", a);
	println!("{}", b);
}
