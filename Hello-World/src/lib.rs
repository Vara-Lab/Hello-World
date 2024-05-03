#![no_std]
use gstd::{msg, prelude::*, debug};

#[no_mangle]
extern "C" fn handle() {
    msg::reply(String::from("Hello"), 0)
        .expect("Error in sending a reply message");
}

#[no_mangle]
extern "C" fn init() {
    let init_message: String = msg::load()
        .expect("Can't load init message");
    debug!("Program was initialized with message {:?}",
        init_message);
}