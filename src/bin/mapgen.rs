use libflategy;
use rand::prelude::*;

/// Demo program for map generation
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut rng = rand::thread_rng();
    let seed = if 2 <= args.len() {
        args[1].parse::<u64>().unwrap()
    } else {
        rng.gen()
    };
    let map = libflategy::core::Map::generate(seed);
    println!("{}", map.dump());
}
