use common::{read_all_lines, IterExt};
use retain_mut::RetainMut;

fn main() {
	let mut t = read_all_lines();
	let n = t.next().unwrap();
	let mut b = t
		.batching(|t| {
			t.next()?;
			let mut s = vec![vec![0u8; 5]; 10];
			for i in 0..5 {
				let l = t.next()?;
				let mut w = l.split(" ").filter(|&w| !w.is_empty());
				for j in 0..5 {
					let v = w.next()?.parse().ok()?;
					s[i][j] = v;
					s[j + 5][i] = v;
				}
			}
			Some(s)
		})
		.collect::<Vec<_>>();
	let m = b.len();

	let (a, b) = n
		.split(",")
		.filter_map(|v| v.parse::<u8>().ok())
		.fold_while((0usize, 0u32, 0u32), |(c, a, d), v| {
			b.retain_mut(|s| {
				let mut o = false;
				for r in s.iter_mut() {
					r.retain(|&x| x != v);
					if r.len() == 0 {
						o = true;
					}
				}
				if o {
					*c += 1;
					if *c == 1 {
						*a = u(s) * v as u32;
					}
					if *c == m {
						*d = u(s) * v as u32;
					}
				}
				!o
			});

			if b.len() == 0 {
				Some((*a, *d))
			} else {
				None
			}
		})
		.unwrap();

	println!("{:#?}", a);
	println!("{:#?}", b);
}

fn u(s: &[Vec<u8>]) -> u32 {
	s.iter()
		.take(5)
		.fold(0, |a, r| a + r.iter().map(|&v| v as u32).sum::<u32>())
}
