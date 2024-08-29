#![no_std]

use goods_io::*;
use goods_services::*;
use gstd::msg;

#[no_mangle]
extern "C" fn handle() {
    let action: GoodsActions = msg::load().expect("Unable to decode GoodsActions");
    let result = goods_action_handler(action);
    msg::reply(result, 0).expect("Failed to send state reply.");
}

// #[no_mangle]
// extern "C" fn handle_reply() {

//     let rely_message_id = msg::reply_to().expect("Failed to get message Id");

//     // let query: BlogStateActions = msg::load().expect("Unable to load the state query");
//     // let result = handle_blog_state(query);
//     msg::reply("result", 0).expect("Failed to send state reply.");
// }

#[no_mangle]
extern "C" fn state() {
    let query: GoodsStateActions = msg::load().expect("Unable to load the state query");
    let result = goods_state_handler(query);
    msg::reply(result, 0).expect("Failed to send state reply.");
}
