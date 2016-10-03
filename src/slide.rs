pub fn slide_permute(i: usize, n: usize) -> usize {
	if i % 2 == 0 { i / 2 }
	else { n - (i + 1) / 2 }
}

pub fn get_cycles(n: usize) -> Vec<Vec<usize>> {
	// list of finished cycles
	let mut cycles = Vec::new();

	// what elements have already been assigned to a cycle
	let mut found = Vec::with_capacity(n);
	for _ in 0..n {
		found.push(false);
	}

	// the 0 cycle is always its own length 1 cycle
	cycles.push(vec!(0));

	if n == 0 { return cycles; }

	'cycles_loop: for start in 1..n {
		let mut i = start;

		let mut cycle = vec!(i);
		found[i] = true;

		loop {
			i = slide_permute(i, n);
			if i == start { break; }
			if found[i] { continue 'cycles_loop; }
			cycle.push(i);
			found[i] = true;
		}

		cycles.push(cycle);
	}

	cycles
}

pub fn get_cycles_info(n: usize) -> Vec<(usize, usize)> {
	// list of finished cycles
	let mut cycles = Vec::new();

	if n == 0 { return cycles; }

	// the 0 cycle is always its own length 1 cycle
	cycles.push((0, 1));

	// to ensure that our outputted cycles are not duplicated, we will ignore
	// any cycles that do not start on their lowest number (only one such
	// ordering of a given cycle can exist)

	// if first is even, then permute(first) < first
	// if first is odd and greater than 2 * n / 3, then permute(first) < first

	// so we can save time by only starting with the odd numbers less than or equal to 2/3rds n
	'cycles_loop: for start in 0..(n / 3 + 1) {
		let start = start * 2 + 1;
		let mut i = start;

		let mut count = 0 as usize;

		loop {
			// i is guaranteed to be odd, permute as such
			i = n - (i + 1) / 2;
			count += 1;

			// how many times can i be divided by 2? each one of these
			// divisions is a further step in the cycle
			let tz = i.trailing_zeros() as usize;

			i >>= tz;
			// i is now odd by definition
			count += tz;

			// this ordering of the cycle did not start with its lowest value
			if i < start { continue 'cycles_loop; }

			// we started with the lowest odd number so we know we did not miss a point when i == start
			if i == start { break; }
		}

		cycles.push((start, count));
	}

	cycles
}

pub fn get_permute_order(n: usize) -> (usize, Vec<(usize, usize)>) {
	let cycles = get_cycles_info(n);

	let mut order = 0;
	for i in 0..cycles.len() {
		if cycles[i].1 > order { order = cycles[i].1 };
	}

	return (order, cycles);
}