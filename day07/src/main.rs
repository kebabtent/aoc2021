use common::{read_values_as, DoubletSum};

fn main() {
	let mut d = read_values_as(',').collect::<Vec<i32>>();
	d.sort_unstable();
	let l = d.len();
	let m = if l % 2 == 0 {
		(d[l / 2 - 1] + d[l / 2]) / 2
	} else {
		d[l / 2]
	};
	let n = d.iter().sum::<i32>() / l as i32;
	let (a, b) = d
		.iter()
		.map(|&v| ((m - v).abs(), (n - v).abs() * ((n - v).abs() + 1) / 2))
		.doublet_sum::<i32, i32>();

	println!("{}", a);
	println!("{}", b);
}
