extern crate serde;
extern crate reqwest;
extern crate percent_encoding;
extern crate tokio;

use matrix_bot_api::{ActiveBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Output {
    count: u32,
    results: Vec<ID>
}

#[derive(Deserialize, Debug)]
struct ID {
    id: u32,
    title: String,
}

#[tokio::main]
async fn geturl(url: &str) -> Result<Output, Error>{
    let request_url = format!("{}", url);
    let response: Output = reqwest::get(&request_url).await?.json().await?;
    Ok(response)
}

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
const APOLOGY: &str = "I'm sorry, senpai. I can't find it (╥﹏╥)";

fn track(string: &String) -> Option<Output> {
    let request_url = &format!("https://tanukitunes.com/api/v1/tracks/?q={}&ordering=title", string.trim());

    let result = geturl(request_url).unwrap();
    if result.count == 0 {
        return None;
    }

    Some(result)
}

fn find_track_by_title(results: &Vec<ID>, title: String) -> u32 {
    match results.iter().find(|&t| t.title == title) {
        None => results[0].id,
        Some(exact) => exact.id,
    }
}

pub fn trackresp(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    let string = utf8_percent_encode(_cmd.trim(), FRAGMENT).to_string();
    let results = track(&string);

    match results {
        None => bot.send_message(&format!("{}", APOLOGY), &message.room, MessageType::TextMessage),
        Some(result) => {
            let final_id = find_track_by_title(&result.results, string);
            let post_url = &format!("https://tanukitunes.com/library/tracks/{}", final_id);
            bot.send_message(&format!("{}", post_url), &message.room, MessageType::TextMessage);
        },
    }

    HandleResult::StopHandling
}

pub fn albresp(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    let string = utf8_percent_encode(_cmd.trim(), FRAGMENT).to_string();
    let request_url = &format!("https://tanukitunes.com/api/v1/albums/?q={}&ordering=title&page=1&page_size=1", string.trim());
    let results = geturl(request_url).unwrap();
    let count = results.count;
    if count == 0 {
        bot.send_message(&format!("{}", APOLOGY), &message.room, MessageType::TextMessage);
    }
    else {
        let final_id = results.results[0].id;
        let post_url = &format!("https://tanukitunes.com/library/albums/{}", final_id);
        bot.send_message(&format!("{}", post_url), &message.room, MessageType::TextMessage);
    }
    HandleResult::StopHandling
}

pub fn artresp(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    let string = utf8_percent_encode(_cmd.trim(), FRAGMENT).to_string();
    let request_url = &format!("https://tanukitunes.com/api/v1/artists/?q={}&ordering=id&page=1&page_size=1", string.trim());
    let results = geturl(request_url).unwrap();
    let count = results.count;
    if count == 0 {
        bot.send_message(&format!("{}", APOLOGY), &message.room, MessageType::TextMessage);
    }
    else {
        let final_id = results.results[0].id;
        let post_url = &format!("https://tanukitunes.com/library/artists/{}", final_id);
        bot.send_message(&format!("{}", post_url), &message.room, MessageType::TextMessage);
    }
    HandleResult::StopHandling
}

pub fn playresp(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    let string = utf8_percent_encode(_cmd.trim(), FRAGMENT).to_string();
    let request_url = &format!("https://tanukitunes.com/api/v1/playlists/?q={}&ordering=id&page=1&page_size=1", string.trim());
    let results = geturl(request_url).unwrap();
    let count = results.count;
    if count == 0 {
        bot.send_message(&format!("{}", APOLOGY), &message.room, MessageType::TextMessage);
    }
    else {
        let final_id = results.results[0].id;
        let post_url = &format!("https://tanukitunes.com/library/playlists/{}", final_id);
        bot.send_message(&format!("{}", post_url), &message.room, MessageType::TextMessage);
    }
    HandleResult::StopHandling
}

#[cfg(test)]
mod tests {
    use super::{track, find_track_by_title};

    fn track_id(title: String) -> u32 {
        find_track_by_title(&track(&title).unwrap().results, title)
    }

    #[test]
    fn test_track() {
        // multiple results
        assert_eq!(track_id(String::from("Der Königsjodler")), 66484);
        // single result
        assert_eq!(track_id(String::from("Ecos Jerezanos")), 48045);
        // incorrect match from prod
        assert_eq!(track_id(String::from("Dry Dry Try")), 42143);
    }
}
