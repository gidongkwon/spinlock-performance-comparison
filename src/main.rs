mod locks;

use std::{sync::Arc, time::Duration};
use std::thread;
use std::time::Instant;

use locks::{BackoffLock, Lock, Spinlock, TASLock, TTASLock};

const THREAD_COUNTS: [u64; 5] = [2, 4, 8, 16, 32];
const REPEAT_COUNT: u32 = 50;
const START: u64 = 1_000_000;
const END: u64 = 5_000_000;
const SEP: &str = "\t";

fn benchmark_lock<L: Lock>(name: &str) {
    print!("{name:12}{SEP}");
    for thread_count in THREAD_COUNTS {
        let mut time_avg: Duration = Duration::new(0, 0);
        for _ in 0..REPEAT_COUNT {
            let mut sum = 0u64;
            let sum_addr = {
                let ptr: *mut u64 = &mut sum;
                ptr as usize
            };
            let lock = Arc::new(L::new());

            let start_time = Instant::now();

            thread::scope(|s| {
                for i in 0..thread_count {
                    let lock_ref = Arc::clone(&lock);
                    let chunk_size = (END - START) / thread_count;
                    let start = START + i * chunk_size;
                    let end = if i == thread_count - 1 {
                        END
                    } else {
                        START + (i + 1) * chunk_size - 1
                    };
                    s.spawn(move || {
                        for i in start..=end {
                            lock_ref.lock();
                            unsafe {
                                let sum_ptr = sum_addr as *mut u64;
                                *sum_ptr += i;
                            }
                            lock_ref.unlock();
                        }
                    });
                }
            });

            let duration = Instant::now() - start_time;
            time_avg += duration;
        }

        time_avg = time_avg / REPEAT_COUNT;
        print!("{:4}{SEP}", time_avg.as_millis() as f64);
    }

    println!();
}

fn no_lock() {
    print!("No Lock{:>5}", SEP);
    for thread_count in THREAD_COUNTS {
        let mut sum = 0u64;
        let sum_addr = {
            let ptr: *mut u64 = &mut sum;
            ptr as usize
        };
        let start_time = Instant::now();

        thread::scope(|s| {
            for i in 0..thread_count {
                let chunk_size = (END - START) / thread_count;
                let start = START + i * chunk_size;
                let end = if i == thread_count - 1 {
                    END
                } else {
                    START + (i + 1) * chunk_size - 1
                };
                s.spawn(move || {
                    for i in start..=end {
                        unsafe {
                            let sum_ptr = sum_addr as *mut u64;
                            *sum_ptr += i;
                        }
                    }
                });
            }
        });

        let duration = Instant::now() - start_time;
        print!("{:4},{}{SEP}", duration.as_millis() as f64, sum);
    }

    println!();
}

fn main() {
    println!("Lock Type{SEP}{}", THREAD_COUNTS.map(|s| { s.to_string() }).join(SEP));
    no_lock();
    benchmark_lock::<Spinlock>("Spinlock");
    benchmark_lock::<TASLock>("TASLock");
    benchmark_lock::<TTASLock>("TTASLock");
    benchmark_lock::<BackoffLock>("BackoffLock");
}
