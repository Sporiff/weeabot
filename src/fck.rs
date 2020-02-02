extern crate rand;
extern crate config;

use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use matrix_bot_api::{ActiveBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};

pub fn fckresp(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {

    let array = [
        "(ﾉಥ益ಥ)ﾉ",
        "(`皿´＃)",
        "凸(`△´＃)",
        "(凸ಠ益ಠ)凸",
        "ψ( ` ∇ ´ )ψ",
        "↑_(ΦwΦ)Ψ",
        "(ᗒᗣᗕ)՞",
        "凸(￣ヘ￣)"
    ];
    let mut rng = rand::thread_rng();
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("botconfig")).unwrap();
    let get = settings.get_str("fck").unwrap();
    let limit = get.parse().unwrap();
    let mut random = rand::thread_rng();
    let number = Uniform::from(0.00..1.00);
    let resp = number.sample(&mut random);
    if resp > limit {
        bot.send_message(&format!("ファック {}", array.choose(&mut rng).unwrap()), &message.room, MessageType::TextMessage);
    }
HandleResult::StopHandling
}
