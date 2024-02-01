use optimistic::optimistic_lock::OptimisticLock;
use optimistic::rwlock::MyRwLock;
use std::env::args;
use std::hint::black_box;
use std::thread;

fn main() {
    // parse command line arguments
    let num_writers = args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(3);
    if num_writers >= 17 {
        println!("Too many writers, please use a number less than 17");
        panic!();
    }

    // let lock = OptimisticLock::new(0);
    let lock = MyRwLock::new(0);

    thread::scope(|s| {
        for _ in 0..num_writers {
            s.spawn(|| {
                for _ in 0..10000 {
                    let result = lock.write(|data| *data = *data + 1);
                    black_box(result);
                }
            });
        }
        for _ in 0..16 - num_writers {
            s.spawn(|| {
                for _ in 0..10000 {
                    let result = lock.read(|data| *data);
                    black_box(result);
                }
            });
        }
    });

    let result = lock.read(|data| *data);
    println!("Result: {}", result);
}
