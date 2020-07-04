extern crate rand;
extern crate config;

use rand::Rng;
use rand::seq::SliceRandom;
use matrix_bot_api::{ActiveBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};
use crate::botconf::Settings;

pub const MODALS: &'static [&str] = &[
    "fuck", "FUCK", "fck", "fug", "feck", "fml", "FML"
];

const FACES: &'static [&str] = &[
    "(ﾉಥ益ಥ)ﾉ", "(`皿´＃)", "凸(`△´＃)", "(凸ಠ益ಠ)凸", "ψ( ` ∇ ´ )ψ", "↑_(ΦwΦ)Ψ", "(ᗒᗣᗕ)՞", "凸(￣ヘ￣)"
];

pub fn fckresp(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {

    let limit = Settings::get_settings().fck.parse().unwrap();
    let random = rand::thread_rng().gen_range(0.00,1.00);
    if random > limit {
        bot.send_message(&format!("ファック {}", FACES.choose(&mut rand::thread_rng()).unwrap()), &message.room, MessageType::TextMessage);
    }
    HandleResult::StopHandling
}
