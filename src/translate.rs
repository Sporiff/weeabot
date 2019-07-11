extern crate matrix_bot_api;
extern crate config;
extern crate ytr;

use matrix_bot_api::{MatrixBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};

pub fn translateme(bot: &MatrixBot, message: &Message, _cmd: &str) -> HandleResult {

    // Fetch the API key from a file in the src directory
    let mut args = std::env::args();
    args.next();

    let mut conf = config::Config::default();
    conf.merge(config::File::with_name("botconfig")).unwrap();
    let key = conf.get_str("translate").unwrap();
    let api = ytr::ApiClient::new(key);

    // Check for the language being used and format the query string
    let text = _cmd.trim().to_string();
    let tolang = &text[..2];
    let translatestring = &text[3..];
    // Return the translated string
    
    let result = api.translate(&format!("{}", translatestring), &format!("{}", tolang))
        .format("plain")
        .get(); 

    let translation = match result {
        Ok(response) => response.text,
        Err(error) => format!("An error has occurred: {:?}", error),
    };

    let translation_esc = translation.replace("\\'", "'");

    bot.send_message(&format!("{}", translation_esc), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}