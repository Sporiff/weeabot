extern crate matrix_bot_api;
extern crate rand;

use matrix_bot_api::{MatrixBot, MessageType};
use rand::seq::SliceRandom;
use matrix_bot_api::handlers::{Message, HandleResult};

pub fn noticeme(bot: &MatrixBot, message: &Message, _cmd: &str) -> HandleResult {
    let array = [
        "(つ≧▽≦)つ",
        "(//ω//)",
        "(*/ω＼)",
        "(⁄⁄>⁄▽⁄<⁄⁄)",
        "(*/▽＼*)",
        "(*ﾉ∀`*)",
        "σ(≧ε≦σ) ♡",
        "(´ ω `♡)",
        "♡( ◡‿◡ )",
        "٩(♡ε♡)۶"
    ];
    let mut rng = rand::thread_rng();
    bot.send_message(&format!("Notice me senpai {}", array.choose(&mut rng).unwrap()), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}
