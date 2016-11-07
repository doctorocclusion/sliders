pub use std::iter::{Iterator, Take};

pub struct SlideIter {
    next_i: u64,
    n: u64
}

impl SlideIter {
    pub fn new(i: u64, n: u64) -> SlideIter {
        SlideIter {
            next_i: i, 
            n: n
        }
    }

    pub fn next_i(&mut self) -> u64 {
        let out = self.next_i;
        self.next_i = if self.next_i % 2 == 0 { self.next_i / 2 }
                      else { self.n - (self.next_i + 1) / 2 };
        out
    }

    pub fn get_deck_size(&self) -> u64 { self.n }

    pub fn reverse(self) -> SlideBackIter { 
        let mut out = SlideBackIter::new(self.next_i, self.n);
        out.next();
        out
    }
}

impl Iterator for SlideIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> { Some(self.next_i()) }
}

pub struct SlideBackIter {
    next_i: u64,
    n: u64
}

impl SlideBackIter {
    pub fn new(i: u64, n: u64) -> SlideBackIter {
        SlideBackIter {
            next_i: i, 
            n: n
        }
    }

    pub fn prev_i(&mut self) -> u64 {
        let out = self.next_i;
        self.next_i = if self.next_i < (self.n + 1) / 2 { self.next_i * 2 }
                      else { (self.n - self.next_i)  * 2 - 1 };
        out
    }

    pub fn get_deck_size(&self) -> u64 { self.n }

    pub fn reverse(self) -> SlideIter { 
        let mut out = SlideIter::new(self.next_i, self.n);
        out.next();
        out
    }
}

impl Iterator for SlideBackIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> { Some(self.prev_i()) }
}

pub struct Cycle {
    start: u64,
    order: u64,
    deck_size: u64
}

impl Cycle {
    pub fn new(n: u64, lowest: u64, order: u64) -> Cycle { 
        Cycle { 
            start: lowest, 
            order: order, 
            deck_size: n
        }
    }

    pub fn iter(&self) -> SlideIter { SlideIter::new(self.start, self.deck_size) }
    pub fn iter_back(&self) -> SlideBackIter { SlideBackIter::new(self.start, self.deck_size) }
    pub fn iter_once(&self) -> Take<SlideIter> { self.iter().take(self.order as usize) }
    pub fn iter_back_once(&self) -> Take<SlideBackIter> { self.iter_back().take(self.order as usize) }
    pub fn get_order(&self) -> u64 { self.order }
    pub fn get_lowest(&self) -> u64 { self.start }
    pub fn get_deck_size(&self) -> u64 { self.deck_size }
}

pub fn get_cycles(n: u64) -> Vec<Cycle> {
    if n == 0 { return vec![]; }

    // list of finished cycles
    // the 0 cycle is always its own length 1 cycle
    let mut cycles = vec![Cycle::new(n, 0, 1)];

    // to ensure that our outputted cycles are not duplicated, we will ignore
    // any cycles that do not start on their lowest number (only one such
    // ordering of a given cycle can exist)

    // if first is even, then permute(first) < first
    // if first is odd and greater than 2 * n / 3, then permute(first) < first

    // so we can save time by only starting with the odd numbers less than or equal to 2/3rds n
    'cycles_loop: for start in 0..(n / 3 + 1) {
        let start = start * 2 + 1;
        let mut i = start;

        let mut count = 0 as u64;

        loop {
            // i is guaranteed to be odd, permute as such
            i = n - (i + 1) / 2;
            count += 1;

            // how many times can i be divided by 2?
            let tz = i.trailing_zeros() as u64;

            i >>= tz; // divide i by 2 until odd
            count += tz; // each one of these divisions is a further step in the cycle
            
            // this ordering of the cycle did not start with its lowest value
            if i < start { continue 'cycles_loop; }

            // we started with the lowest odd number so we know we did not miss a point when i == start
            if i == start { break; }
        }

        cycles.push(Cycle::new(n, start, count));
    }

    cycles
}