#[macro_use]
extern crate lazy_static;

mod gpu_bruteforcer;
mod arg_lib;

fn main() {
    let key: Vec<usize> = "XDYOYOY".as_bytes().into_iter().map(|x| (*x - 64) as usize).collect();
    let keywords: Vec<&[u8]> = ["test", "passed", "alright"].iter().map(|x| x.as_bytes()).collect();
    
    println!("Keywords: {:?}", keywords);
    // gpu_bruteforcer::start();
    // println!("Result: {}", arg_lib::l2a_accurate(&arg_lib::STATIC_DATA3_VEC, &key, &keywords));
    println!("\nResult: {}", arg_lib::l2a_inaccurate(&arg_lib::STATIC_DATA4_VEC, &key, &keywords));
    return;
    /* 
    let mut handles = Vec::new();
    // it's as shrimple as that
    for begining in 24..25 as usize {
        // Ignore this wierdness. This is for testing only
        println!("Starting thread: {}", begining);
        handles.push(thread::spawn(move || {
            for a in 1..27 as usize {
                for b in 1..27 as usize {
                    for c in 1..27 as usize {
                        for d in 1..27 as usize {
                            for e in 1..27 as usize {
                                for f in 1..27 as usize {
                                    let key: [usize; 7] = [begining, a, b, c, d, e, f];
                                    decrypt(key);
                                }
                            }
                        }
                    }
                }
            }
        }));
    }
    println!("Waiting for the threads");
    for handle in handles {
        handle.join().unwrap();
    }*/
    
}