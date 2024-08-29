#![no_std]

use gstd::msg;
use store_io::*;
use store_services::*;

#[no_mangle]
extern "C" fn handle() {
    let action: StoreGoodsActions = msg::load().expect("Unable to decode GoodsActions");
    let result = store_action_handler(action);
    msg::reply(result, 0).expect("Failed to send state reply.");
}

#[no_mangle]
extern "C" fn state() {
    let query: StoreGoodsStateActions = msg::load().expect("Unable to load the state query");
    let result = store_state_handler(query);
    msg::reply(result, 0).expect("Failed to send state reply.");
}
