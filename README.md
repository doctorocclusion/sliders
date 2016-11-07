A really bloody fast Rust program for calculating the permutations shown in [SlidePermute](http://gh.drocclusion.net/SlidePermute/). It finds the cycles of the size 10,000,000 permutation in about 0.18 seconds. All the magic happens in the ```get_cycles``` function in ```src/slide.rs```.

To run the program:

1. Clone the repository
2. Download and install [Rust](https://www.rust-lang.org/).
3. Open the repo and call ```cargo run --release```.
4. Type deck size, hit enter, and get the cyles!