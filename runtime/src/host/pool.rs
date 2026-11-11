//! Threads that stay up for the whole run, for work split across cores.
//!
//! Glide flushes several times a frame. Starting a thread for each core at
//! every flush cost more than some of the flushes' drawing.

use std::sync::{Arc, Condvar, Mutex};

/// A job's address, with its lifetime removed. `run` waits until every worker
/// is done with it, so it never outlives the borrow it came from.
#[derive(Clone, Copy)]
struct Job(*const (dyn Fn() + Sync));
unsafe impl Send for Job {}

struct Shared {
    job: Option<Job>,
    /// Counts jobs, so a worker can tell a new one from the one it just did.
    generation: u64,
    /// Workers still on the current job.
    busy: usize,
}

pub struct Pool {
    shared: Arc<(Mutex<Shared>, Condvar)>,
    workers: usize,
}

impl Pool {
    /// A pool that runs each job on `threads` threads, the caller's included.
    pub fn new(threads: usize) -> Pool {
        let shared = Arc::new((Mutex::new(Shared { job: None, generation: 0, busy: 0 }), Condvar::new()));
        let workers = threads.max(1) - 1;
        for _ in 0..workers {
            let shared = Arc::clone(&shared);
            std::thread::spawn(move || {
                let (lock, wake) = &*shared;
                let mut seen = 0;
                loop {
                    let job = {
                        let mut state = lock.lock().unwrap();
                        while state.generation == seen {
                            state = wake.wait(state).unwrap();
                        }
                        seen = state.generation;
                        state.job.unwrap()
                    };
                    // A panic still counts as done, or `run` would wait forever.
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { (*job.0)() }));
                    let mut state = lock.lock().unwrap();
                    state.busy -= 1;
                    if state.busy == 0 {
                        wake.notify_all();
                    }
                }
            });
        }
        Pool { shared, workers }
    }

    pub fn threads(&self) -> usize {
        self.workers + 1
    }

    /// Runs `job` on every thread at once, and returns when all have finished.
    /// The job shares out the work itself, such as by taking bands from a queue.
    pub fn run(&self, job: &(dyn Fn() + Sync)) {
        if self.workers == 0 {
            return job();
        }
        let (lock, wake) = &*self.shared;
        {
            let mut state = lock.lock().unwrap();
            // Erases the lifetime: the wait below keeps the borrow alive.
            state.job = Some(Job(unsafe { std::mem::transmute::<&(dyn Fn() + Sync), &'static (dyn Fn() + Sync)>(job) }));
            state.generation += 1;
            state.busy = self.workers;
            wake.notify_all();
        }
        // Waits for the workers even if the job panics here, since they still
        // hold the borrow.
        struct Finish<'a>(&'a (Mutex<Shared>, Condvar));
        impl Drop for Finish<'_> {
            fn drop(&mut self) {
                let (lock, wake) = self.0;
                let mut state = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                while state.busy > 0 {
                    state = wake.wait(state).unwrap_or_else(|poisoned| poisoned.into_inner());
                }
                state.job = None;
            }
        }
        let _finish = Finish(&self.shared);
        job();
    }
}
