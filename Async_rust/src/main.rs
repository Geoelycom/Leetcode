#![allow(unused_variables)]
fn main() {
    let number = get_number();
    // println!("Hello, your number is {:?}!", number);
}

async fn get_number() -> u8 {
    return 10
}
// when we set an async into a front of a function we are changing that function return call into what we call "FUTURE"

// AFTER future is called, we need to call it with an await keyword for it to actually return to us that function.