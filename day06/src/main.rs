use common::read_lines;

fn main() {
	let mut c = [0u64; 9];
	let l = read_lines().next().unwrap();
	for f in l.split(",").filter_map(|v| v.parse::<usize>().ok()) {
		c[f] += 1;
	}

	let mut h = 6;
	println!("{}", g(&mut h, &mut c, 80));
	println!("{}", g(&mut h, &mut c, 256 - 80));
}

fn g(h: &mut usize, c: &mut [u64; 9], n: u32) -> u64 {
	for _ in 0..n {
		*h = (*h + 1) % 9;
		c[*h] += c[(*h + 2) % 9];
	}
	c.iter().sum::<u64>()
}
