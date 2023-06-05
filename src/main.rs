use std::{
    thread::{self, JoinHandle},
    time::Instant, sync::{Mutex, Arc},
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
    let prime_numbers_arc = Arc::new(Mutex::new(vec![]));
    for i in 1..100000 {
        let prime_numbers_mutex = Arc::clone(&prime_numbers_arc);
        let handle = thread::spawn(move || {
            if is_prime(i) {
                let mut prime_numbers = prime_numbers_mutex.lock().unwrap();
                prime_numbers.push(i);
                drop(prime_numbers);
                // println!("{} is prime", i);
            } else {
                // println!("{} is not prime", i);
            }
        });
        join_handles.push(handle);

        if i % 10000 == 0 {
            join_handles.into_iter().for_each(|join_handle| {
                join_handle.join().unwrap();
            });

            join_handles = vec![];
        }
    }

    print!("Prime numbers are {:?}", prime_numbers_arc);

    let elapsed = now.elapsed();
    println!("Time to execute {:?}", elapsed);
}

// fn main() {
//     let now = Instant::now();
//     let mut prime_numbers = vec![];
//     for i in 1..100000 {
//             if is_prime(i) {
//                 prime_numbers.push(i);
//                 // println!("{} is prime", i);
//             } else {
//                 // println!("{} is not prime", i)
//             }
//     }
//     print!("Prime numbers are {:?}", prime_numbers);

//     let elapsed = now.elapsed();
//     println!("Time to execute {:?}", elapsed);
// }
