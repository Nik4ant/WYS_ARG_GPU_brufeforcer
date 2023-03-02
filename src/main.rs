#[macro_use]
extern crate lazy_static;

mod gpu_bruteforcer;
mod arg_lib;

fn main() {
    let key: Vec<usize> = "XDYOYOY".as_bytes().into_iter().map(|x| (*x - 64) as usize).collect();
    println!("Key: {:?}", key);
    println!("Using DATA4; DATA4 length: {}; Expected l2a output length: {}", arg_lib::DATA4_LEN, 26);
    // println!("DATA4 ({}): {:?}", arg_lib::STATIC_DATA3_VEC.len(), arg_lib::STATIC_DATA3_VEC.as_slice());
    // let keywords: Vec<&[u8]> = ["test", "passed", "alright"].iter().map(|x| x.as_bytes()).collect();
    
    // println!("Keywords: {:?}", keywords);
    gpu_bruteforcer::start_bruteforcer_sync();
    // println!("Result: {}", arg_lib::l2a_accurate(&arg_lib::STATIC_DATA3_VEC, &key, &keywords));
    // println!("\nResult: {}", arg_lib::l2a_inaccurate(&arg_lib::STATIC_DATA4_VEC, &key, &keywords));
}