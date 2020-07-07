use matrix_bot_api::handlers::{HandleResult, Message};
use matrix_bot_api::{ActiveBot, MessageType};
use rand::seq::SliceRandom;

pub const MODALS: &'static [&str] = &[
    "can", "could", "may", "might", "shall", "should", "will", "would", "must", "ought", "are",
    "am", "is", "does", "did", "didn't", "do", "don't", "dont", "was", "didnt",
];
const RESPONSES: &'static [&str] = &[
    "yes",
    "no",
    "definitely",
    "absolutely",
    "never",
    "nope",
    "nah",
    "yeah",
];

pub fn yes_no(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    bot.send_message(
        &format!("{}", RESPONSES.choose(&mut rand::thread_rng()).unwrap()),
        &message.room,
        MessageType::TextMessage,
    );
    HandleResult::StopHandling
}

fn choose_item(text: &str) -> &str {
    text.split(',')
        .filter(|&choice| choice.len() > 0)
        .map(|choice| choice.trim())
        .collect::<Vec<&str>>()
        .choose(&mut rand::thread_rng())
        .unwrap()
}

pub fn choose_resp(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    if _cmd.trim().chars().all(|c| c == ',' || c == ' ') {
        bot.send_message(
            "Please send me valid input senpai ﴾͡๏̯͡๏﴿",
            &message.room,
            MessageType::TextMessage,
        );
    } else {
        match _cmd.find(',') {
            None => bot.send_message(
                "I couldn't find multiple choices senpai (๑′°︿°๑)",
                &message.room,
                MessageType::TextMessage,
            ),
            Some(_) => bot.send_message(choose_item(_cmd), &message.room, MessageType::TextMessage),
        }
    }

    HandleResult::StopHandling
}

#[cfg(test)]
mod tests {
    use super::choose_item;

    #[test]
    fn test_choose_item() {
        let result1 = choose_item("franzl, jodlerkönig");
        assert!(result1 == "franzl" || result1 == "jodlerkönig");

        let result2 = choose_item("franzl,");
        assert_eq!(result2, "franzl");
    }
}
