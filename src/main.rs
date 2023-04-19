use std::{
    thread::{self, JoinHandle},
    time::Instant,
};

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
    let mut join_handles: Vec<JoinHandle<()>> = vec![];
    for i in 1..10000 {
        let handle = thread::spawn(move || {
            if is_prime(i) {
                println!("{} is prime", i);
            } else {
                println!("{} is not prime", i)
            }
        });
        join_handles.push(handle);
    }

    join_handles.into_iter().for_each(|join_handle| {
        join_handle.join().unwrap();
    });

    let elapsed = now.elapsed();
    println!("Time to execute {:?}", elapsed);
}
