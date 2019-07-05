extern crate config;
extern crate matrix_bot_api;

mod rip;
mod pat;
mod romakana;
mod notice;
mod translate;

use matrix_bot_api::MatrixBot;
use matrix_bot_api::handlers::StatelessHandler;
use rip::ripresp;
use pat::headpat;
use romakana::{kanaconvert, romaconvert};
use notice::noticeme;
use translate::translateme;

fn main() {

// Get settings from Toml file

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("src/botconfig")).unwrap();

    let user = settings.get_str("user").unwrap();
    let password  = settings.get_str("password").unwrap();
    let homeserver_url = settings.get_str("homeserver_url").unwrap();

// Senpai function

    let mut notice = StatelessHandler::new();
    notice.set_cmd_prefix("%");
    notice.register_handle("senpai", noticeme);

    let mut bot = MatrixBot::new(notice);

// Translate function

    let mut tran = StatelessHandler::new();
    tran.set_cmd_prefix("%");
    tran.register_handle("translate", translateme);

    bot.add_handler(tran);

// Kana function

    let mut kana = StatelessHandler::new();
    kana.set_cmd_prefix("%");
    kana.register_handle("kana", kanaconvert);

    bot.add_handler(kana);

// Romaji function

    let mut roma = StatelessHandler::new();
    roma.set_cmd_prefix("%");
    roma.register_handle("roma", romaconvert);

    bot.add_handler(roma);

// Headpat function

    let mut head = StatelessHandler::new();
    head.set_cmd_prefix("%");
    head.register_handle("headpat", headpat);

    bot.add_handler(head);

// Rip function

    let mut rip = StatelessHandler::new();
    rip.set_cmd_prefix("");
    rip.register_handle("rip", ripresp);
    rip.register_handle("rippu", ripresp);
    rip.register_handle("rup", ripresp);

    bot.add_handler(rip);

// Login function

    bot.run(&user, &password, &homeserver_url);
}