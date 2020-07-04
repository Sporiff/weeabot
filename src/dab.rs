extern crate matrix_bot_api;
extern crate rand;
extern crate config;

use rand::Rng;
use matrix_bot_api::{ActiveBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};
use rand::seq::SliceRandom;
use crate::botconf::Settings;

const FACES: &'static [&str] = &[
    "ヽ(o⌣oヾ)", "ヽ( •_)ᕗ", "／ʕ •ᴥ•ʔ／"
];

pub fn senddab(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {

    let limit = Settings::get_settings().dab.parse().unwrap();
    let random = rand::thread_rng().gen_range(0.00,1.00);
    if random > limit {
        bot.send_message(&format!("{}", FACES.choose(&mut rand::thread_rng()).unwrap()), &message.room, MessageType::TextMessage);
    }
    HandleResult::StopHandling
}
