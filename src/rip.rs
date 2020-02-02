extern crate rand;
extern crate config;

use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use matrix_bot_api::{ActiveBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};

pub fn ripresp(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {

    let array = [
        "(ノ_<。)",
        "( ╥ω╥ )",
        "(╥_╥)",
        "(ಥ﹏ಥ)",
        "(｡•́︿•̀｡)",
        "( ; ω ; )",
        "o(TヘTo)",
        "(´-ω-`)",
        "｡･ﾟﾟ*(>д<)*ﾟﾟ･｡",
        "(╥﹏╥)",
        ".･ﾟﾟ･(／ω＼)･ﾟﾟ･."
    ];
    let mut rng = rand::thread_rng();
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("botconfig")).unwrap();

    let get = settings.get_str("rip").unwrap();
    let limit = get.parse().unwrap();
    let mut random = rand::thread_rng();
    let number = Uniform::from(0.00..1.00);
    let resp = number.sample(&mut random);
    if resp > limit {
        bot.send_message(&format!("リップ {}", array.choose(&mut rng).unwrap()), &message.room, MessageType::TextMessage);
    }
    HandleResult::StopHandling
}
