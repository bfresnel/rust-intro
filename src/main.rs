use std::thread;
use std::time::{Duration, Instant};
use rand::{Rng, thread_rng};

fn main() {
    let mut random_thread = thread_rng();
    let thread_pool_limit = random_thread.gen_range(1..11);

    println!("Number of activated thread(s) : {}", thread_pool_limit);

    for i in 0..thread_pool_limit {
        thread::spawn(move||{
            let now = Instant::now();
            for _x in 0..101 {
                let mut random_ms = thread_rng();
                let random_value = random_ms.gen_range(0..101);
                //println!("[THREAD-{}] Loading... {}%", i, x);
                thread::sleep(Duration::from_millis(random_value));
            };
            println!("THREAD {} finished in {} ms", i + 1, now.elapsed().as_millis());
        });
    }

    thread::sleep(Duration::from_secs(10));
}
