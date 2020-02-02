extern crate matrix_bot_api;
extern crate rand;
extern crate config;

use matrix_bot_api::{ActiveBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;

pub fn senddab(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {

    let array = [
        "ヽ(o⌣oヾ)",
        "ヽ( •_)ᕗ",
        "／ʕ •ᴥ•ʔ／"
    ];
    let mut rng = rand::thread_rng();
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("botconfig")).unwrap();

    let get = settings.get_str("dab").unwrap();
    let limit = get.parse().unwrap();
    let mut random = rand::thread_rng();
    let number = Uniform::from(0.00..1.00);
    let resp = number.sample(&mut random);
    if resp > limit {
        bot.send_message(&format!("{}", array.choose(&mut rng).unwrap()), &message.room, MessageType::TextMessage);
    }
    HandleResult::StopHandling
}
