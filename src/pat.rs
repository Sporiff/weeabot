extern crate matrix_bot_api;
extern crate rand;

use matrix_bot_api::{ActiveBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};
use rand::seq::SliceRandom;


pub fn headpat(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    let array = [
        "(*≧ω≦*)",
        "(≧◡≦)",
        "o(≧▽≦)o",
        "(´｡• ω •｡`)",
        "(っ˘ω˘ς )",
        "ヽ(o＾▽＾o)ノ",
        "ヽ(>∀<☆)ノ",
        "(=^-ω-^=)",
        "(^˵◕ω◕˵^)",
        "(=^‥^=)",
        "(=^･ｪ･^=)"
    ];
    let mut rng = rand::thread_rng();
    bot.send_message(&format!("nyan nyan {}", array.choose(&mut rng).unwrap()), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}
