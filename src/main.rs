use std::{thread, time::Instant};

fn is_prime(n: u32) -> bool {
    let mut is_prime = true;
    for i in 2..(n - 1) {
        if n % i == 0 {
            is_prime = false;
        }
    }
    is_prime
}

fn main() {
    let now = Instant::now();
    for i in 1..10000 {
        let handle = thread::spawn(move || {
            if is_prime(i) {
                println!("{} is prime", i);
            } else {
                println!("{} is not prime", i)
            }
        });
        handle.join().unwrap();
    }

    let elapsed = now.elapsed();
    println!("Time to execute {:?}", elapsed);
}
