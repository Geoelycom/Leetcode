// CHANNELS
// A channel is a way to send (data) multiple values from one thread to another. One part of your code calls send with a value, and another part checks for values with recv. Channels are useful when you have a lot of work to do in a thread and want to send the results to another thread for processing.
// a channel has two parts, a receiver and a sender. The sender is used to send messages, and the receiver is used to receive messages.


use std::sync::mpsc;
use std::thread;
// use std::time::Duration;
fn main() {
   let (tx, rx) = mpsc::channel();
   // mpsc stands for multiple producer, single consumer. This means that you can have multiple threads sending messages to one thread. The tx variable is the sending half of the channel, and the rx variable is the receiving half of the channel.
   // the mpsc channel returns a turple with the first element of which being the sending half(transmitter) of the channel and the second element being the receiving(reciever) half of the channel.
   thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
   });
   let recieved = rx.recv().unwrap();
   println!("Got: {recieved}");
   // The reciever has two useful methods, recv, and try_recv.
}


// Recv = Blocking wait for a Message
// Try_recv = Non-blocking wait for a message

// Channels and Ownership Transference

// The ownership of the value is transferred from the sender to the receiver. This means that after the value is sent, the sender can no longer use the value. This is because the value is moved into the channel and then moved from the channel to the receiver. This is similar to how ownership works with functions and return values.


// fn main_1() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//         // println!("val is {val}"); this item as already been moved, hence the error we would recieve from here
//     });

//     let received = rx.recv().unwrap();
//     println!("Got: {received}");
// }

// Note: varaible Reference is not possible inside channels. we know that the send method inside the tx gives ownership to rcv thanks to the move method insidet the closure. to access the val inside the above code, we can clone method or access the val inside the recv where it was moved into. hence this is the reason why the val is not accessible inside the closure.

// // tx.send(&val).unwrap();  // This does NOT work!
// 🚨 Why this fails:

// Channels are used for inter-thread communication.
// References (&val) are not safe to send between threads because they might become invalid if the sender thread exits.
// The Rust compiler requires values sent through channels to have 'static lifetime or be owned (hence the move).


// fn main_2 () {
//     let (tx, rx) = mpsc::channel();
//     let thread = thread::spawn(move || {
//     let vals = vec![
//         String::from("hi"),
//         String::from("from"),
//         String::from("the"),
//         String::from("thread"),
//     ];
//     for val in vals {
//         tx.send(val).unwrap();
//         thread::sleep(Duration::from_secs(1));
//     }
// });
// for recieved in rx {
//     println!("Got: {recieved}");
// }
// // or a try_recv
// // loop {
// //     match rx.try_recv() {
// //         Ok(received) => println!("Got: {received}"),
// //         Err(_) => {
// //             println!("No message yet...");
// //             thread::sleep(Duration::from_millis(500));
// //         }
// //     }
// // }


// }

// Creating Multiple Producers by Cloning the Transmitter
// fn main_3 () {
// let (tx, rx) = mpsc::channel();
// let tx1 = tx.clone();
// thread::spawn(move || {
//     let vals = vec![
//         String::from("hi"),
//         String::from("from"),
//         String::from("the"),
//         String::from("thread"),
//     ];
//     for val in vals {
//         tx1.send(val).unwrap();
//         thread::sleep(Duration::from_secs(1));
//     }
// });

// thread::spawn(move || {
//     let vals = vec![
//         String::from("more"),
//         String::from("messages"),
//         String::from("for"),
//         String::from("you"),
//     ];
//     for val in vals {
//         tx.send(val).unwrap();
//         thread::sleep(Duration::from_secs(1));
//     }
// });
// for received in rx {
//     println!("Got: {received}");
// }
// }