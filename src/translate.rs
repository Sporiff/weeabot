extern crate matrix_bot_api;

use matrix_bot_api::{MatrixBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};
use wana_kana::is_japanese::*;

pub fn translateme(bot: &MatrixBot, message: &Message, _cmd: &str) -> HandleResult {
    let input = _cmd.trim().to_string();
    let testlang = is_japanese(&input);
    let lang =
        if testlang { "ja" }
        else { "en" };
    bot.send_message(&format!("{}", lang), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}