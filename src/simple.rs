use matrix_bot_api::handlers::{Message, HandleResult};
use matrix_bot_api::{ActiveBot, MessageType};
use rand::seq::SliceRandom;
use rand::Rng;
use crate::botconf::Settings;

pub const RIPMODALS: &'static [&str] = &[
    "rip", "rippo", "rippu", "rup", "RIP", "Rip", "rop",
];

pub const FCKMODALS: &'static [&str] = &[
    "fuck", "FUCK", "fck", "fug", "feck", "fml", "FML"
];

const FCKFACE: &'static [&str] = &[
    "(ﾉಥ益ಥ)ﾉ", "(`皿´＃)", "凸(`△´＃)", "(凸ಠ益ಠ)凸", "ψ( ` ∇ ´ )ψ", "↑_(ΦwΦ)Ψ", "(ᗒᗣᗕ)՞", "凸(￣ヘ￣)"
];

const RIPFACE: &'static [&str] = &[
    "(ノ_<。)", "( ╥ω╥ )", "(╥_╥)", "(ಥ﹏ಥ)", "(｡•́︿•̀｡)", "( ; ω ; )", "o(TヘTo)", "(´-ω-`)", "｡･ﾟﾟ*(>д<)*ﾟﾟ･｡", "(╥﹏╥)", ".･ﾟﾟ･(／ω＼)･ﾟﾟ･."
];

const SENPAIFACE: &'static [&str] = &[
    "(つ≧▽≦)つ", "(//ω//)", "(*/ω＼)", "(⁄⁄>⁄▽⁄<⁄⁄)", "(*/▽＼*)", "(*ﾉ∀`*)", "σ(≧ε≦σ) ♡", "(´ ω `♡)", "♡( ◡‿◡ )", "٩(♡ε♡)۶"
];

const HEADPATFACE: &'static [&str] = &[
    "(*≧ω≦*)", "(≧◡≦)", "o(≧▽≦)o", "(´｡• ω •｡`)", "(っ˘ω˘ς )", "ヽ(o＾▽＾o)ノ", "ヽ(>∀<☆)ノ", "(=^-ω-^=)", "(^˵◕ω◕˵^)", "(=^‥^=)", "(=^･ｪ･^=)"
];

const DABFACE: &'static [&str] = &[
    "ヽ(o⌣oヾ)", "ヽ( •_)ᕗ", "／ʕ •ᴥ•ʔ／"
];

pub fn senddab(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {

    let limit = Settings::get_settings().dab.parse().unwrap();
    let random = rand::thread_rng().gen_range(0.00,1.00);
    if random > limit {
        bot.send_message(&format!("{}", DABFACE.choose(&mut rand::thread_rng()).unwrap()), &message.room, MessageType::TextMessage);
    }
    HandleResult::StopHandling
}

pub fn headpat(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    bot.send_message(&format!("nyan nyan {}", HEADPATFACE.choose(&mut rand::thread_rng()).unwrap()), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}

pub fn noticeme(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    bot.send_message(&format!("Notice me senpai {}", SENPAIFACE.choose(&mut rand::thread_rng()).unwrap()), &message.room, MessageType::TextMessage);
    HandleResult::StopHandling
}

pub fn ripresp(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {

    let limit = Settings::get_settings().rip.parse().unwrap();
    let random = rand::thread_rng().gen_range(0.00,1.00);
    if random > limit {
        bot.send_message(&format!("リップ {}", RIPFACE.choose(&mut rand::thread_rng()).unwrap()), &message.room, MessageType::TextMessage);
    }
    HandleResult::StopHandling
}

pub fn fckresp(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {

    let limit = Settings::get_settings().fck.parse().unwrap();
    let random = rand::thread_rng().gen_range(0.00,1.00);
    if random > limit {
        bot.send_message(&format!("ファック {}", FCKFACE.choose(&mut rand::thread_rng()).unwrap()), &message.room, MessageType::TextMessage);
    }
    HandleResult::StopHandling
}