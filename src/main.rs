extern crate time;

mod slide;

use time::{PreciseTime};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        let line = lines.next().unwrap().unwrap();
        let n = line.parse::<i64>().unwrap() as usize;

        let start = PreciseTime::now();
        let cycles = slide::get_cycles(n);
        let order = cycles.iter()
            .map(|ref i| i.get_order())
            .max()
            .unwrap();
        let start = start.to(PreciseTime::now());

        println!("    Done in {} ms", start.num_milliseconds());
        println!("    Permutation #{} has order {} and {} cycles with lengths:", n, order, cycles.len());
        for c in cycles.iter().take(100) {
            let mut cit = c.iter();
            let mut cbt = c.iter_back();
            cbt.prev_i();
            match c.get_order() {
                1 => println!("        {} ({})", c.get_order(), cit.next_i()),
                2 => println!("        {} ({}, {})", c.get_order(), cit.next_i(), cit.next_i()),
                3 => println!("        {} ({}, {}, {})", c.get_order(), cit.next_i(), cit.next_i(), cit.next_i()),
                4 => println!("        {} ({}, {}, {}, {})", c.get_order(), cit.next_i(), cit.next_i(), cit.next_i(), cit.next_i()),
                5 => println!("        {} ({}, {}, {}, {}, {})", c.get_order(), cit.next_i(), cit.next_i(), cit.next_i(), cit.next_i(), cit.next_i()),
                _ => println!("        {} ({}, {}, {}, ..., {5}, {4})", c.get_order(), cit.next_i(), cit.next_i(), cit.next_i(), cbt.prev_i(), cbt.prev_i())
            }
        }
        if (cycles.len() > 100) {
            println!("        ... ({} more)", cycles.len() - 100); 
        }
        println!("");
    }
}