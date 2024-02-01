use std::sync::RwLock;

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
