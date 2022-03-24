use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicU8, Ordering};

pub struct NotOnceCell<T> {
    status: AtomicU8,
    inner: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T> Send for NotOnceCell<T> {}
unsafe impl<T> Sync for NotOnceCell<T> {}

#[repr(u8)]
enum Status {
    Uninitialized,
    Initializing,
    Initialized,
}

impl<T> NotOnceCell<T> {
    pub const fn new() -> Self {
        Self {
            status: AtomicU8::new(0),
            inner: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub fn get_or_init<F>(&self, init: F) -> &T
    where
        F: FnOnce() -> T,
    {
        if self
            .status
            .compare_exchange(
                Status::Uninitialized as u8,
                Status::Initializing as u8,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_ok()
        {
            unsafe {
                (*self.inner.get()).write(init());

                self.status
                    .store(Status::Initialized as u8, Ordering::Release);

                (*self.inner.get()).assume_init_ref()
            }
        } else if self.status.load(Ordering::Relaxed) == Status::Initialized as u8 {
            unsafe { (*self.inner.get()).assume_init_ref() }
        } else {
            loop {
                if self
                    .status
                    .compare_exchange(
                        Status::Initializing as u8,
                        Status::Initialized as u8,
                        Ordering::Acquire,
                        Ordering::Relaxed,
                    )
                    .is_ok()
                {
                    break;
                }
            }

            unsafe { (*self.inner.get()).assume_init_ref() }
        }
    }
}

impl<T> Drop for NotOnceCell<T> {
    fn drop(&mut self) {
        if self.status.load(Ordering::Acquire) == Status::Initialized as u8 {
            unsafe { (*self.inner.get()).assume_init_drop() }
        }
    }
}
