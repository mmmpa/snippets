// rand = "0.6.5"

use rand::thread_rng;

fn r(range: Range<u32>) -> Vec<u32> {
    let mut vec: Vec<u32> = range.collect();
    vec.shuffle(&mut thread_rng());
    vec
}
