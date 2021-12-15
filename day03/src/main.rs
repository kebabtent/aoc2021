use common::read_lines;
use std::iter::once;

fn main() {
	let mut it = read_lines();
	let f = it.next().unwrap();
	let m = f.len();
	let d = once(f)
		.chain(it)
		.filter_map(|l| u32::from_str_radix(&l, 2).ok())
		.collect::<Vec<_>>();
	let n = d.len() as u32;

	let x = (0..m)
		.map(|i| d.iter().map(|&v| (v >> i) & 1).sum::<u32>())
		.map(|v| v > n / 2)
		.enumerate()
		.filter(|&(_, v)| v)
		.map(|(i, _)| 1 << i)
		.sum::<u32>();
	let a = x * (!x & ((1 << m) - 1));
	let b = r(m, d.clone(), true) * r(m, d, false);

	println!("{}", a);
	println!("{}", b);
}

fn r(m: usize, mut d: Vec<u32>, f: bool) -> u32 {
	let mut i = 1;
	while d.len() > 1 {
		let x = d.iter().map(|&v| (v >> (m - i)) & 1).sum::<u32>() >= (d.len() as u32 + 1) / 2;
		d.retain(|&v| ((v >> (m - i)) & 1 == 1) == (x == f));
		i += 1;
	}
	d[0]
}
