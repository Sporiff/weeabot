extern crate matrix_bot_api;

use matrix_bot_api::{MatrixBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};

pub fn headpat(bot: &MatrixBot, message: &Message, _cmd: &str) -> HandleResult {
    bot.send_message("nyan nyan (=^-ω-^=)", &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}