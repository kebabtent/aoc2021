use common::{read_all_lines, IterExt};

fn main() {
	let mut t = read_all_lines();
	let n = t.next().unwrap();
	let mut d = t
		.batching(|t| {
			t.next()?;
			let mut s = [0u64; 11];
			for i in 0..5 {
				let l = t.next()?;
				let w = l
					.split(" ")
					.filter(|&w| !w.is_empty())
					.filter_map(|w| w.parse::<u64>().ok())
					.enumerate();
				for (j, v) in w {
					s[i + 1] += v + 1 << 8 * j;
					s[j + 6] += v + 1 << 8 * i;
				}
			}
			Some(s)
		})
		.collect::<Vec<_>>();
	let m = d.len();

	let (a, b) = n
		.split(",")
		.filter_map(|v| v.parse::<u64>().ok())
		.map(|v| v + 1)
		.fold_while((0, 0, 0), |(c, a, b), v| {
			for s in d.iter_mut().filter(|s| s[0] == 0) {
				let mut o = false;
				for r in s.iter_mut().skip(1) {
					for i in 0..5 {
						if (*r >> 8 * i) & 0xFF == v {
							*r -= v << 8 * i;
						}
					}
					if *r == 0 {
						o = true;
					}
				}
				if o {
					s[0] = 1;
					*c += 1;
					if *c == 1 || *c == m {
						let u = s.iter().skip(1).take(5).fold(0, |a, &r| {
							a + (0..5).fold(0, |a, i| a + ((r >> 8 * i) & 0xFF).saturating_sub(1))
						}) * (v - 1);
						if *c == 1 {
							*a = u;
						} else {
							*b = u;
						}
					}
				}
			}

			if *c == m {
				Some((*a, *b))
			} else {
				None
			}
		})
		.unwrap();

	println!("{}", a);
	println!("{}", b);
}
