use std::cell::UnsafeCell;
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;

pub struct OptimisticLock<T> {
    version: AtomicU64,
    write_lock: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> OptimisticLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            version: AtomicU64::new(0),
            write_lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn read<U, F>(&self, mut f: F) -> U
    where
        F: FnMut(&T) -> U,
    {
        loop {
            let version = self.version.load(Ordering::Acquire);
            // Safety: We're only reading data, and we check for data integrity after.
            let data = unsafe { &*self.data.get() };
            let result = f(data);

            // Check if the version has changed during the function call
            if self.version.load(Ordering::Acquire) == version {
                return result;
            }
            // If the version has changed, loop to retry
        }
    }

    pub fn write<U, F>(&self, mut f: F) -> U
    where
        F: FnMut(&mut T) -> U,
    {
        while self.write_lock.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }

        self.version.fetch_add(1, Ordering::AcqRel);
        // Safety: We have exclusive access thanks to the write lock.
        let data = unsafe { &mut *self.data.get() };
        let result = f(data);

        self.write_lock.store(false, Ordering::Release);
        result
    }
}

unsafe impl<T> Sync for OptimisticLock<T> where T: Send + Sync {}

fn main() {
    let lock = OptimisticLock::new(0);

    thread::scope(|s| {
        for _ in 0..8 {
            s.spawn(|| {
                for _ in 0..100000 {
                    let result = lock.read(|data| *data);
                    black_box(result);
                }
            });
        }
        for _ in 0..5 {
            s.spawn(|| {
                for _ in 0..100000 {
                    let result = lock.write(|data| *data = *data + 1);
                    black_box(result);
                }
            });
        }
    });

    let result = lock.read(|data| *data);
    println!("Result: {}", result);
}
