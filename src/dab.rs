extern crate matrix_bot_api;
extern crate rand;

use matrix_bot_api::{MatrixBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};
use rand::distributions::{Distribution, Uniform};

pub fn senddab(bot: &MatrixBot, message: &Message, _cmd: &str) -> HandleResult {
    let mut random = rand::thread_rng();
    let number = Uniform::from(0.00..1.00);
    let resp = number.sample(&mut random);
    if resp > 0.30 {
        bot.send_message("／ʕ •ᴥ•ʔ／", &message.room, MessageType::TextMessage);
    }
    HandleResult::StopHandling
}