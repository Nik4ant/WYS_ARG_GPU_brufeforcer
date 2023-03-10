#[macro_use]
extern crate lazy_static;

mod gpu_bruteforcer;
mod arg_lib;


fn main() {
    let key: Vec<usize> = "XDYOYOY".as_bytes().into_iter().map(|x| (*x - 64) as usize).collect();
    println!("Key: {:?}", key);
    println!("Unaligned DATA: [u8; {}]", arg_lib::DATA4.len());
    println!("Aligned DATA: [[u32; 4]; {}] ({} items total)", arg_lib::GPU_ALIGNED_DATA4.len(), arg_lib::GPU_ALIGNED_DATA4.len() * 4);
    
    gpu_bruteforcer::start_bruteforcer_sync();
   
}