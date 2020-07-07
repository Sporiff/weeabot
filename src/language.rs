use matrix_bot_api::{ActiveBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};
use wana_kana::to_romaji::to_romaji;
use wana_kana::to_kana::to_kana;
use crate::botconf::Settings;

// Convert from romaji to kana

pub fn romaconvert(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    let input = &_cmd.trim().to_string();
    bot.send_message(&format!("{}", to_romaji(input)), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}

// Convert from kana to romaji

pub fn kanaconvert(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    let input = &_cmd.trim().to_string();
    bot.send_message(&format!("{}", to_kana(input)), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}

// Translate a string given provided language code

fn translatephrase(lang: &str, string: &str) -> String {
    // Fetch the API key from a file in the src directory
    let key = Settings::get_settings().translate;
    let api = ytr::ApiClient::new(key);

    // Return the translated string
    let result = api.translate(&format!("{}", string), &format!("{}", lang))
        .format("plain")
        .get();

    let translation = match result {
        Ok(response) => response.text,
        Err(error) => format!("Sorry Senpai... {:?}", error),
    };

    let translation_esc = translation.replace("\"", "");
    return translation_esc;
}

// Sanitize input and return string

pub fn translateme(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {

    // Check for the language being used and format the query string
    let command = _cmd.trim().to_string();
    let text: Vec<&str> = command.split(" ").collect();
    let tolang = &text[0];
    let translatestring = &text[1..].join(" ");

    bot.send_message(&format!("{}", translatephrase(tolang, translatestring)), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}

// Translation Tests

#[cfg(test)]
mod tests {
    use super::translatephrase;

    #[test]
    fn test_translation() {
        assert_eq!(translatephrase("fr", "hello"), "bonjour");
        assert_eq!(translatephrase("fr-de", "j'adore franzl lang"), "ich liebe franzl lang");
        assert_eq!(translatephrase("ja-fp", "hello"), "Sorry Senpai... ApiError(ApiError { code: 501, message: The specified translation direction is not supported })");
        assert_eq!(translatephrase("fp", "hello"), "Sorry Senpai... ApiError(ApiError { code: 501, message: The specified translation direction is not supported })");
        assert_eq!(translatephrase("fhidewohbfeowb", ""), "Sorry Senpai... ApiError(ApiError { code: 502, message: Invalid parameter: lang })");
    }
}
