use matrix_bot_api::{ActiveBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};
use rand::seq::SliceRandom;

pub const MODALS: &'static [&str] = &[
    "can", "could", "may", "might", "shall", "should", "will", "would",
    "must", "ought", "are", "am", "is", "does", "did", "didn't", "do",
    "don't", "dont", "was", "didnt",
];
const RESPONSES: &'static [&str] = &[
    "yes", "no", "definitely", "absolutely", "never", "nope", "nah", "yeah",
];

pub fn yes_no(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    bot.send_message(
        &format!("{}", RESPONSES.choose(&mut rand::thread_rng()).unwrap()),
        &message.room, MessageType::TextMessage
    );
    HandleResult::StopHandling
}
