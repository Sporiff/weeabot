extern crate matrix_bot_api;
extern crate rand;

use matrix_bot_api::{ActiveBot, MessageType};
use rand::seq::SliceRandom;
use matrix_bot_api::handlers::{Message, HandleResult};

const FACES: &'static [&str] = &[
    "(つ≧▽≦)つ", "(//ω//)", "(*/ω＼)", "(⁄⁄>⁄▽⁄<⁄⁄)", "(*/▽＼*)", "(*ﾉ∀`*)", "σ(≧ε≦σ) ♡", "(´ ω `♡)", "♡( ◡‿◡ )", "٩(♡ε♡)۶"
];

pub fn noticeme(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    bot.send_message(&format!("Notice me senpai {}", FACES.choose(&mut rand::thread_rng()).unwrap()), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}
