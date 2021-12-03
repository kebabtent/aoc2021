use common::{read_lines, IterExt};

fn main() {
	let (x, y, z) = read_lines().fold((0, 0, 0), |(x, y, z), l| {
		let (c, v) = l.split(" ").next_tuple().unwrap();
		let v = v.parse::<u32>().unwrap();
		match c {
			"forward" => (x + v, y, z + y * v),
			"down" => (x, y + v, z),
			"up" => (x, y - v, z),
			_ => unreachable!(),
		}
	});

	println!("{}", x * y);
	println!("{}", x * z);
}
