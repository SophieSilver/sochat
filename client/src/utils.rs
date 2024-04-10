pub mod deadlock_detect {
    //! A module that provides a deadlock detecting `Mutex` type
    //! that panics when the lock is acquired twice on the same thread.

    use std::{
        fmt::Display,
        num::NonZeroUsize,
        ops::{Deref, DerefMut},
        sync::{
            atomic::{AtomicUsize, Ordering},
            Mutex as StdMutex, MutexGuard as StdMutexGuard,
        },
    };

    /// A `Mutex` lock with deadlock detection
    #[derive(Debug, Default)]
    pub struct Mutex<T> {
        // if set to 0, no thread is holding the lock
        holding_thread: AtomicUsize,
        inner: StdMutex<T>,
    }

    impl<T> Mutex<T> {
        pub const fn new(data: T) -> Self
        where
            T: Sized,
        {
            Self {
                holding_thread: AtomicUsize::new(0),
                inner: StdMutex::new(data),
            }
        }

        pub fn lock(&self) -> MutexGuard<'_, T> {
            // checking the id and locking aren't happening atomically together
            // so, in theory the id might change between us checking it and trying to lock the mutex
            // however, we only care if the id is the same as the current thread's,
            // so this is fine, since it's impossible to call it at the same time from the same thread
            if self.holding_thread.load(Ordering::Relaxed) == thread_id().get() {
                panic!("Mutex locked twice in the same thread");
            }

            let guard = self.inner.lock().unwrap();
            self.holding_thread
                .store(thread_id().get(), Ordering::Relaxed);

            MutexGuard {
                holding_thread: &self.holding_thread,
                inner: guard,
            }
        }
    }

    #[derive(Debug)]
    pub struct MutexGuard<'a, T: 'a> {
        holding_thread: &'a AtomicUsize,
        inner: StdMutexGuard<'a, T>,
    }

    impl<T: Display> Display for MutexGuard<'_, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.fmt(f)
        }
    }

    impl<T> Deref for MutexGuard<'_, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl<T> DerefMut for MutexGuard<'_, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }

    impl<T> Drop for MutexGuard<'_, T> {
        fn drop(&mut self) {
            self.holding_thread.store(0, Ordering::Relaxed);
        }
    }

    /// Get a number uniquely identifying each thread
    ///
    /// This is different from std::thread::ThreadId, because I would prefer to have a usize rather than an opaque type.
    fn thread_id() -> NonZeroUsize {
        // see https://github.com/Amanieu/parking_lot/blob/master/src/remutex.rs#L18-L27
        thread_local! (static KEY: u8 = 0);

        KEY.with(|x| x as *const _ as usize)
            .try_into()
            .expect("Thread ID is zero")
    }

    #[cfg(test)]
    mod tests {
        use std::thread;

        use super::Mutex;

        #[test]
        fn deadlock_test() {
            let mutex = Mutex::new(42);
            let thread_result = thread::scope(|s| {
                s.spawn(|| {
                    let _guard1 = mutex.lock();
                    let _guard2 = mutex.lock();
                })
                .join()
            });
            assert!(thread_result.is_err());
        }
    }
}
