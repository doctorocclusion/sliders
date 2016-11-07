extern crate time;

mod slide;

use time::{PreciseTime};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        let line = lines.next().unwrap().unwrap();
        let n = line.parse::<u64>().unwrap();

        let start = PreciseTime::now();
        let cycles = slide::get_cycles(n);
        let order = cycles.iter()
            .map(|ref i| i.get_order())
            .max()
            .unwrap();
        let start = start.to(PreciseTime::now());

        println!("    Done in {} ms", start.num_milliseconds());
        println!("    Permutation #{} has order {} and {} cycles with lengths:", n, order, cycles.len());
        for c in cycles.iter() {
            println!("        {} ({})", c.get_order(), c.iter_once().fold(String::new(), |msg, i| msg + &i.to_string() + " "));
        }
        println!("");
    }
}