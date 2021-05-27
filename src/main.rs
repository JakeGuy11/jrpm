extern crate rand;
use rand::Rng;

fn main() {
    let key_len = rand::thread_rng().gen_range(20..50);
    println! ("len: {}", key_len);
}
