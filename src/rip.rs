extern crate rand;

use rand::distributions::{Distribution, Uniform};
use matrix_bot_api::{MatrixBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};

pub fn ripresp(bot: &MatrixBot, message: &Message, _cmd: &str) -> HandleResult {
    let mut random = rand::thread_rng();
    let number = Uniform::from(0.00..1.00);
    let resp = number.sample(&mut random);
    if resp > 0.60 {
        bot.send_message("リップ (´-ω-`)", &message.room, MessageType::TextMessage);
    }
    HandleResult::StopHandling
}