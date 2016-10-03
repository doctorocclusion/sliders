extern crate time;

mod slide;

use time::{PreciseTime};
use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	loop {
   		let line = stdin.lock().lines().next().unwrap().unwrap();
   		let n = line.parse::<i64>().unwrap() as usize;

   		let start = PreciseTime::now();
   		let out = slide::get_permute_order(n);
   		let start = start.to(PreciseTime::now());

   		println!("    Done in {} ms", start.num_milliseconds());
		println!("    Permutation #{} has order {} and {} cycles with lengths:", n, out.0, out.1.len());
		print!("  ");
		for c in out.1 {
			print!("  {} ({}...)", c.1, c.0);
		}
		println!("");
	}
}