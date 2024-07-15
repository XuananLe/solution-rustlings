// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // Create an Arc<Mutex<JobStatus>> for shared access across threads
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    // Spawn 10 threads
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            // Simulate work by sleeping for 250 milliseconds
            thread::sleep(Duration::from_millis(250));
            // Lock the Mutex before updating the shared value
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // Wait for all threads to complete their work
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of jobs_completed
    println!("jobs completed {}", status.lock().unwrap().jobs_completed);
}