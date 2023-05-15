use log::info;
use rand::{thread_rng, Rng};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let mut random_thread = thread_rng();
    let thread_pool_limit = random_thread.gen_range(1..11);

    info!("Number of activated thread(s) : {}", thread_pool_limit);

    for i in 0..thread_pool_limit {
        thread::spawn(move || {
            let now = Instant::now();
            for _x in 0..101 {
                let mut random_ms = thread_rng();
                let random_value = random_ms.gen_range(0..101);
                //debug!("[THREAD-{}] Loading... {}%", i, x);
                thread::sleep(Duration::from_millis(random_value));
            }
            info!(
                "THREAD {} finished in {} ms",
                i + 1,
                now.elapsed().as_millis()
            );
        });
    }

    thread::sleep(Duration::from_secs(10));
}
