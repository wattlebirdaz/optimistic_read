use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

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
        let mut version = self.version.load(Ordering::Acquire);
        let mut current_version;
        loop {
            // Safety: We're only reading data, and we check for data integrity after.
            let data = unsafe { &*self.data.get() };
            let result = f(data);

            // compare the version again to check if it has changed
            current_version = self.version.load(Ordering::Acquire);
            if version == current_version {
                return result;
            } else {
                version = current_version;
            }
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
