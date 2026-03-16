//! # Mutex Shared State
//!
//! In this exercise, you will use `Arc<Mutex<T>>` to safely share and modify data between multiple threads.
//!
//! ## Concepts
//! - `Mutex<T>` mutex protects shared data
//! - `Arc<T>` atomic reference counting enables cross-thread sharing
//! - `lock()` acquires the lock and accesses data

use std::sync::{Arc, Mutex};
use std::thread;

/// Increment a counter concurrently using `n_threads` threads.
/// Each thread increments the counter `count_per_thread` times.
/// Returns the final counter value.
///
/// Hint: Use `Arc<Mutex<usize>>` as the shared counter.
pub fn concurrent_counter(n_threads: usize, count_per_thread: usize) -> usize {
    // Create Arc<Mutex<usize>> with initial value 0
    // Spawn n_threads threads
    // In each thread, lock() and increment count_per_thread times
    // Join all threads, return final value
    let shared_counter = Arc::new(Mutex::new(0usize));

    thread::scope(|s| {
        for _ in 0..n_threads {
            let shared_counter = Arc::clone(&shared_counter);
            s.spawn(move || {
                let mut counter = shared_counter.lock().unwrap();
                *counter += count_per_thread;
            });
        }
    });

    Arc::try_unwrap(shared_counter)
        .unwrap()
        .into_inner()
        .unwrap()
}

/// Add elements to a shared vector concurrently using multiple threads.
/// Each thread pushes its own id (0..n_threads) to the vector.
/// Returns the sorted vector.
///
/// Hint: Use `Arc<Mutex<Vec<usize>>>`.
pub fn concurrent_collect(n_threads: usize) -> Vec<usize> {
    // Create Arc<Mutex<Vec<usize>>>
    // Each thread pushes its own id
    // After joining all threads, sort the result and return
    let shared_vector = Arc::new(Mutex::new(Vec::new()));

    thread::scope(|s| {
        for i in 0..n_threads {
            let shared_vector = Arc::clone(&shared_vector);
            s.spawn(move || {
                shared_vector.lock().unwrap().push(i);
            });
        }
    });

    let mut result = Arc::try_unwrap(shared_vector)
        .unwrap()
        .into_inner()
        .unwrap();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_single_thread() {
        assert_eq!(concurrent_counter(1, 100), 100);
    }

    #[test]
    fn test_counter_multi_thread() {
        assert_eq!(concurrent_counter(10, 100), 1000);
    }

    #[test]
    fn test_counter_zero() {
        assert_eq!(concurrent_counter(5, 0), 0);
    }

    #[test]
    fn test_collect() {
        let result = concurrent_collect(5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_collect_single() {
        assert_eq!(concurrent_collect(1), vec![0]);
    }
}
