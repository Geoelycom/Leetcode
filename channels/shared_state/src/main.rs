// SHARED STATE CONCURENCY
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m, {m:?}!");
    // do not communicate by sharing memory; instead, share memory by communicating
}


// MUTEX = mutual exclusive
// Mutexes have a reputation for being difficult to use because you have to remember two rules:

// You must attempt to acquire the lock before using the data.
// When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
