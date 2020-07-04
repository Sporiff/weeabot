extern crate rand;
extern crate config;

use rand::seq::SliceRandom;
use rand::Rng;
use matrix_bot_api::{ActiveBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};
use crate::botconf::Settings;

pub const MODALS: &'static [&str] = &[
    "rip", "rippo", "rippu", "rup", "RIP", "Rip", "rop",
];

const FACES: &'static [&str] = &[
    "(ノ_<。)", "( ╥ω╥ )", "(╥_╥)", "(ಥ﹏ಥ)", "(｡•́︿•̀｡)", "( ; ω ; )", "o(TヘTo)", "(´-ω-`)", "｡･ﾟﾟ*(>д<)*ﾟﾟ･｡", "(╥﹏╥)", ".･ﾟﾟ･(／ω＼)･ﾟﾟ･."
];

pub fn ripresp(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {

    let limit = Settings::get_settings().rip.parse().unwrap();
    let random = rand::thread_rng().gen_range(0.00,1.00);
    if random > limit {
        bot.send_message(&format!("リップ {}", FACES.choose(&mut rand::thread_rng()).unwrap()), &message.room, MessageType::TextMessage);
    }
    HandleResult::StopHandling
}
