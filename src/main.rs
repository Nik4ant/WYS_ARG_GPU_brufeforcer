#[macro_use]
extern crate lazy_static;

mod gpu_bruteforcer;
mod arg_lib;

fn main() {
    let key: Vec<usize> = "XDYOYOY".as_bytes().into_iter().map(|x| (*x - 64) as usize).collect();
    println!("{:?}", key);
    // let keywords: Vec<&[u8]> = ["test", "passed", "alright"].iter().map(|x| x.as_bytes()).collect();
    
    // println!("Keywords: {:?}", keywords);
    gpu_bruteforcer::start_bruteforcer_sync();
    // println!("Result: {}", arg_lib::l2a_accurate(&arg_lib::STATIC_DATA3_VEC, &key, &keywords));
    // println!("\nResult: {}", arg_lib::l2a_inaccurate(&arg_lib::STATIC_DATA4_VEC, &key, &keywords));
}