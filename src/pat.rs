extern crate matrix_bot_api;
extern crate rand;

use matrix_bot_api::{ActiveBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};
use rand::seq::SliceRandom;

const FACES: &'static [&str] = &[
    "(*≧ω≦*)", "(≧◡≦)", "o(≧▽≦)o", "(´｡• ω •｡`)", "(っ˘ω˘ς )", "ヽ(o＾▽＾o)ノ", "ヽ(>∀<☆)ノ", "(=^-ω-^=)", "(^˵◕ω◕˵^)", "(=^‥^=)", "(=^･ｪ･^=)"
];

pub fn headpat(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    bot.send_message(&format!("nyan nyan {}", FACES.choose(&mut rand::thread_rng()).unwrap()), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}
