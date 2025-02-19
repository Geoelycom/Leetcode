use std::thread;
use std::time::Duration;

fn main() {
   let handle =  thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

// Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running
// we fix the premature or not running of the spawn thread by calling handle.join().unwrap(); and saving the return value of the spawn thread into a varaible to make sure the main thread waits for the spawned thread to finish.

//  when we move the handle.join().unwrap() before the loop, the main thread will wait for the spawned thread to finish before it starts its loop.

// Small details, such as where join is called, can affect whether or not your threads run at the same time.
