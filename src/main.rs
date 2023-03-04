#[macro_use]
extern crate lazy_static;

mod gpu_bruteforcer;
mod arg_lib;


fn main() {
    let key: Vec<usize> = "XDYOYOY".as_bytes().into_iter().map(|x| (*x - 64) as usize).collect();
    println!("Key: {:?}", key);
    gpu_bruteforcer::start_bruteforcer_sync();
}