use tokio::net::{ TcpListener, TcpStream };
use mini_redis::{ Connection, Frame };
use mini_redis::Command::{self, Get, Set};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
  // Bind the Listener to the address("localhost:6379")
  let listener = TcpListener::bind("localhost:6379").await.unwrap();
  println!("Listening");

  let db = Arc::new(Mutex::new(HashMap::new()));
  
  loop {
    // the second item contains the ip and the port of the new connection
    let (socket, _) = listener.accept().await.unwrap();
    let db = db.clone();

    println!("Accepted");
    tokio::spawn(async move {
      process(socket, db).await;
    });
  }
}


async fn process(socket: TcpStream, db: Db) {
   // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.

    let mut connection = Connection::new(socket);

    while let Some(frame) =  connection.read_frame().await.unwrap() {
      let response = match Command::from_frame(frame).unwrap() {
        Set(cmd) => {
          // The value is stored as `Vec<u8>`
          let mut db = db.lock().unwrap();
          db.insert(cmd.key().to_string(), cmd.value().clone());
          Frame::Simple("OK".to_string())
        }
        Get(cmd) => {
          let db = db.lock().unwrap();
          if let Some(value) = db.get(cmd.key()) {
            // `Frame::Bulk` expects data to be of type `Bytes`. This
            // type will be covered later in the tutorial. For now,
            // `&Vec<u8>` is converted to `Bytes` using `into()`.
            Frame::Bulk(value.clone())
        } else {
            Frame::Null
        }
    }
    // Respond with an error
      cmd => panic!("unimplemented {:?}", cmd),
        };
         // Write the response to the client
         connection.write_frame(&response).await.unwrap();
      }
    }

// A tokio spawn task always return a Joinhandle, and we can await on the joinhandle to return to us a result we can now work with.