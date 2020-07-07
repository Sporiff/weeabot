use matrix_bot_api::{ActiveBot, MessageType};
use matrix_bot_api::handlers::{Message, HandleResult};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::Deserialize;
use reqwest::Error;
use crate::botconf::Settings;

// Define layout of returned results

#[derive(Deserialize, Debug)]
struct Output {
    count: u32,
    results: Vec<ID>
}

#[derive(Deserialize, Debug)]
struct ID {
    id: u32,
    #[serde(default)]
    title: String,
    #[serde(default)]
    name: String,
}

// Fetch JSON response from server

#[tokio::main]
async fn geturl(url: &str) -> Result<Output, Error>{
    let request_url = format!("{}", url);
    let response: Output = reqwest::get(&request_url).await?.json().await?;
    Ok(response)
}

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
const APOLOGY: &str = "I'm sorry, senpai. I can't find it (╥﹏╥)";
const BLANK: &str = "Silly senpai. I need something to search for ┐('～`;)┌";

// Pass query parameter to endpoint

fn track(string: &String) -> Option<Output> {
    let request_url = &format!("{}/api/v1/tracks/?q={}&ordering=title", Settings::get_settings().funkwhale_url, string.trim());

    let result = geturl(request_url).unwrap();
    if result.count == 0 {
        return None;
    }

    Some(result)
}

fn album(string: &String) -> Option<Output> {
    let request_url = &format!("{}/api/v1/albums/?q={}&ordering=title", Settings::get_settings().funkwhale_url, string.trim());

    let result = geturl(request_url).unwrap();
    if result.count == 0 {
        return None;
    }

    Some(result)
}

fn artist(string: &String) -> Option<Output> {
    let request_url = &format!("{}/api/v1/artists/?q={}&ordering=name", Settings::get_settings().funkwhale_url, string.trim());

    let result = geturl(request_url).unwrap();
    if result.count == 0 {
        return None;
    }

    Some(result)
}

// Pattern match results

fn find_track_by_title(results: &Vec<ID>, title: String) -> u32 {
    match results.iter().find(|&t| t.title.to_lowercase() == title.to_lowercase()) {
        None => results[0].id,
        Some(exact) => exact.id,
    }
}

fn find_album_by_title(results: &Vec<ID>, title: String) -> u32 {
    match results.iter().find(|&t| t.title.to_lowercase() == title.to_lowercase()) {
        None => results[0].id,
        Some(exact) => exact.id,
    }
}

fn find_artist_by_name(results: &Vec<ID>, name: String) -> u32 {
    match results.iter().find(|&t| t.name.to_lowercase() == name.to_lowercase()) {
        None => results[0].id,
        Some(exact) => exact.id,
    }
}

// Handle and return track search results

pub fn trackresp(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    let query = _cmd.trim().to_string().to_lowercase();

    if query == "" {
        bot.send_message(&format!("{}", BLANK), &message.room, MessageType::TextMessage);
    }

    else {
        let string = utf8_percent_encode(_cmd.trim(), FRAGMENT).to_string().to_lowercase();
        let results = track(&string);

        match results {
            None => bot.send_message(&format!("{}", APOLOGY), &message.room, MessageType::TextMessage),
            Some(result) => {
                let final_id = find_track_by_title(&result.results, query);
                let post_url = &format!("{}/library/tracks/{}", Settings::get_settings().funkwhale_url, final_id);
                bot.send_message(&format!("{}", post_url), &message.room, MessageType::TextMessage);
            },
        }
    }

    HandleResult::StopHandling
}

// Handle and return album search results

pub fn albresp(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    let query = _cmd.trim().to_string().to_lowercase();

    if query == "" {
        bot.send_message(&format!("{}", BLANK), &message.room, MessageType::TextMessage);
    }
    else {
        let string = utf8_percent_encode(_cmd.trim(), FRAGMENT).to_string().to_lowercase();
        let results = album(&string);

        match results {
            None => bot.send_message(&format!("{}", APOLOGY), &message.room, MessageType::TextMessage),
            Some(result) => {
                let final_id = find_album_by_title(&result.results, query);
                let post_url = &format!("{}/library/albums/{}", Settings::get_settings().funkwhale_url, final_id);
                bot.send_message(&format!("{}", post_url), &message.room, MessageType::TextMessage);
            },
        }
    }

    HandleResult::StopHandling
}

// Handle and return artist search results

pub fn artresp(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    let query = _cmd.trim().to_string().to_lowercase();

    if query == "" {
        bot.send_message(&format!("{}", BLANK), &message.room, MessageType::TextMessage);
    }
    else {
        let string = utf8_percent_encode(_cmd.trim(), FRAGMENT).to_string().to_lowercase();
        let results = artist(&string);

        match results {
            None => bot.send_message(&format!("{}", APOLOGY), &message.room, MessageType::TextMessage),
            Some(result) => {
                let final_id = find_artist_by_name(&result.results, query);
                let post_url = &format!("{}/library/artists/{}", Settings::get_settings().funkwhale_url, final_id);
                bot.send_message(&format!("{}", post_url), &message.room, MessageType::TextMessage);
            },
        }
    }

    HandleResult::StopHandling
}

// Query tests

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

    use super::{album, find_album_by_title};

    fn album_id(title: String) -> u32 {
        find_album_by_title(&album(&title).unwrap().results, title)
    }

    #[test]
    fn test_album() {
        //multiple results
        assert_eq!(album_id(String::from("Sleeping in Traffic")), 5583);
        // single result
        assert_eq!(album_id(String::from("Alpen-Echo")), 7662);
        //incorrect match from prod
        assert_eq!(album_id(String::from("Poni Hoax")), 8123);
    }

    use super::{artist, find_artist_by_name};

    fn artist_id(name: String) -> u32 {
        find_artist_by_name(&artist(&name).unwrap().results, name)
    }

    #[test]
    fn test_artist() {
        // single result
        assert_eq!(artist_id(String::from("Franzl Lang")), 8557);
        //incorrect match from prod
        assert_eq!(artist_id(String::from("Sparks")), 6558);
    }
}
