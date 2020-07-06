extern crate matrix_bot_api;
extern crate rand;

use matrix_bot_api::{ActiveBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};
use rand::seq::SliceRandom;

const COIN: &'static [&str] = &[
    "Heads", "Tails"
];

pub fn headstails(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    bot.send_message(&format!("{}", COIN.choose(&mut rand::thread_rng()).unwrap()), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}
