use matrix_bot_api::{ActiveBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};
use wana_kana::to_romaji::*;
use wana_kana::to_kana::*;
use crate::botconf::Settings;

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

pub fn translateme(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {

    // Fetch the API key from a file in the src directory
    let key = Settings::get_settings().translate;
    let api = ytr::ApiClient::new(key);

    // Check for the language being used and format the query string
    let command = _cmd.trim().to_string();
    let text: Vec<&str> = command.split(" ").collect();
    let tolang = &text[0];
    let translatestring = &text[1..].join(" ");

    // Return the translated string
    let result = api.translate(&format!("{:?}", translatestring), &format!("{}", tolang))
        .format("plain")
        .get();

    let translation = match result {
        Ok(response) => response.text,
        Err(error) => format!("Sorry Senpai... {:?}", error),
    };

    let translation_esc = translation.replace("\"", "");

    bot.send_message(&format!("{}", translation_esc), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}
