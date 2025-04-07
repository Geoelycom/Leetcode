// SHARED STATE CONCURENCY
use std::sync::Mutex;
use bytes::Bytes;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: Bytes,
    }
}



#[tokio::main]
 async fn main() {
    // create an mpsc channel
    let (tx, mut rx) = mpsc::channel(32);
    // The multi-producer capability allows messages to be sent from many tasks. Creating the channel returns two values, a sender and a receiver. The two handles are used separately. They may be moved to different tasks.

    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await.unwrap();
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }


    // It is not possible to clone the receiver of an mpsc channel.



    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // println!("m, {m:?}!");
    // do not communicate by sharing memory; instead, share memory by communicating
}


// MUTEX = mutual exclusive
// Mutexes have a reputation for being difficult to use because you have to remember two rules:

// You must attempt to acquire the lock before using the data.
// When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.


// tokio provides multiple ways to solve the the message passing problem
// 1. mpsc = multiple producers, single consumer channel
// 2. oneshot = single producer, single consumer channel.
// 3. broadcast = multiple producers, multi consumers. many values can be sent. each receiver sees every value
// 4. watch: multi-producer, multi-consumer. Many values can be sent, but no history is kept. Receivers only see the most recent value.

// The mpsc channel stores elements in blocks. Blocks are organized in a linked list. Sending pushes new elements onto the block at the front of the list, and receiving pops them off the one at the back. A block can hold 32 messages on a 64-bit target and 16 messages on a 32-bit target. This number is independent of channel and message size. Each block also stores 4 pointer-sized values for bookkeeping (so on a 64-bit machine, each message has 1 byte of overhead).

// When all values in a block have been received, it becomes empty. It will then be freed, unless the channel’s first block (where newly-sent elements are being stored) has no next block. In that case, the empty block is reused as the next block.

