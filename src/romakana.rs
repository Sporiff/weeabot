use matrix_bot_api::handlers::{Message, HandleResult};
use matrix_bot_api::{ActiveBot, MessageType};
use wana_kana::to_romaji::*;
use wana_kana::to_kana::*;

pub fn romaconvert(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    let input = &_cmd.trim().to_string();
    bot.send_message(&format!("{}", to_romaji(input)), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}

pub fn kanaconvert(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    let input = &_cmd.trim().to_string();
    bot.send_message(&format!("{}", to_kana(input)), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}
