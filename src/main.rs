extern crate config;
extern crate matrix_bot_api;
extern crate wana_kana;

use matrix_bot_api::{MatrixBot, MessageType};
use matrix_bot_api::handlers::{Message, StatelessHandler, HandleResult};
use wana_kana::to_kana::*;
use wana_kana::to_romaji::*;

fn main() {

// Get settings from Toml file

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("src/botconfig")).unwrap();

    let user = settings.get_str("user").unwrap();
    let password  = settings.get_str("password").unwrap();
    let homeserver_url = settings.get_str("homeserver_url").unwrap();

// Senpai function

    let mut who = StatelessHandler::new();
    who.set_cmd_prefix("%");
    who.register_handle("senpai", whoareyou);

    let mut bot = MatrixBot::new(who);
    
// Meso function
    
    let mut meso = StatelessHandler::new();
    meso.set_cmd_prefix("%");
    meso.register_handle("meso", mesohorny);
    
    bot.add_handler(meso);

// Translate function

    let mut tran = StatelessHandler::new();
    tran.set_cmd_prefix("%");
    tran.register_handle("translate", translateme);

    bot.add_handler(tran);

// Kana function

    let mut kana = StatelessHandler::new();
    kana.set_cmd_prefix("%");
    kana.register_handle("kana", kanaconvert);

    bot.add_handler(kana);

// Romaji function

    let mut roma = StatelessHandler::new();
    roma.set_cmd_prefix("%");
    roma.register_handle("roma", romaconvert);

    bot.add_handler(roma);

// Login function
    
    bot.run(&user, &password, &homeserver_url);
}

// Define return messages

fn whoareyou(bot: &MatrixBot, message: &Message, _cmd: &str) -> HandleResult {
    bot.send_message("Notice me senpai (´ω｀*)", &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}

fn mesohorny(bot: &MatrixBot, message: &Message, _cmd: &str) -> HandleResult {
    bot.send_message("Me so horny (๑♡⌓♡๑)", &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}

fn translateme(bot: &MatrixBot, message: &Message, _cmd: &str) -> HandleResult {
    let input = _cmd.trim().to_string();
    bot.send_message(&format!("{}", input), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}

fn kanaconvert(bot: &MatrixBot, message: &Message, _cmd: &str) -> HandleResult {
    let input = &_cmd.trim().to_string();
    let result = to_kana(input);
    bot.send_message(&format!("{}", result), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}

fn romaconvert(bot: &MatrixBot, message: &Message, _cmd: &str) -> HandleResult {
    let input = &_cmd.trim().to_string();
    let result = to_romaji(input);
    bot.send_message(&format!("{}", result), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}
