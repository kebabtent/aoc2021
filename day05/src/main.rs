use common::{read_lines, IterExt};
use std::collections::HashMap;
use std::mem;

fn main() {
	let d = read_lines()
		.filter_map(|l| {
			let mut w = l
				.split(&[',', ' '][..])
				.filter_map(|v| v.parse::<u32>().ok());
			let mut p = w.next_tuple::<(_, _)>()?;
			let mut q = w.next_tuple::<(_, _)>()?;
			if p.0 > q.0 || p.1 > q.1 {
				mem::swap(&mut p, &mut q);
			}
			Some((p, q))
		})
		.fold(HashMap::with_capacity(1024), |mut d, (mut p, mut q)| {
			if p.0 == q.0 || p.1 == q.1 {
				for x in p.0..=q.0 {
					for y in p.1..=q.1 {
						let n = d.entry((x, y)).or_insert((0u8, 0u8));
						n.0 += 1;
					}
				}
			} else {
				if p.0 > q.0 {
					mem::swap(&mut p, &mut q);
				}
				let mut y = p.1;
				for x in p.0..=q.0 {
					let n = d.entry((x, y)).or_insert((0u8, 0u8));
					n.1 += 1;
					if q.1 > p.1 {
						y += 1;
					} else {
						y -= 1;
					}
				}
			}
			d
		});

	println!("{}", d.iter().filter(|&(_, (n, _))| *n > 1).count());
	println!("{}", d.iter().filter(|&(_, (n, m))| *n + *m > 1).count());
}
