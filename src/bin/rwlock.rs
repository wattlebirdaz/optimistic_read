use std::hint::black_box;
use std::sync::RwLock;
use std::thread;

pub struct MyRwLock<T> {
    lock: RwLock<T>,
}

impl<T> MyRwLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            lock: RwLock::new(data),
        }
    }

    pub fn read<U, F>(&self, f: F) -> U
    where
        F: FnOnce(&T) -> U,
    {
        let guard = self.lock.read().unwrap();
        f(&guard)
    }

    pub fn write<U, F>(&self, f: F) -> U
    where
        F: FnOnce(&mut T) -> U,
    {
        let mut guard = self.lock.write().unwrap();
        f(&mut guard)
    }
}

fn main() {
    let lock = MyRwLock::new(0);

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
