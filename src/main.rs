extern crate config;
extern crate serde;
extern crate rand;
extern crate wana_kana;
extern crate ytr;
extern crate reqwest;
extern crate percent_encoding;
extern crate tokio;

mod rip;
mod pat;
mod romakana;
mod notice;
mod translate;
mod dab;
mod fck;
mod fw;
mod botconf;
mod choices;
mod ht;

use matrix_bot_api::MatrixBot;
use matrix_bot_api::handlers::StatelessHandler;
use rip::ripresp;
use fck::fckresp;
use fw::{artresp, trackresp, albresp};
use pat::headpat;
use romakana::{kanaconvert, romaconvert};
use notice::noticeme;
use translate::translateme;
use dab::senddab;
use botconf::Settings;
use choices::yes_no;
use ht::headstails;

fn main() {

// Senpai function

    let mut notice = StatelessHandler::new();
    notice.set_cmd_prefix("%");
    notice.register_handle("senpai", noticeme);

    let mut bot = MatrixBot::new(notice);

// Dab function

    let mut dab = StatelessHandler::new();
    dab.set_cmd_prefix("");
    dab.register_handle("dab", senddab);

    bot.add_handler(dab);

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


// Heads/Tails function

    let mut ht = StatelessHandler::new();
    ht.set_cmd_prefix("%");
    ht.register_handle("ht", headstails);

    bot.add_handler(ht);

// Rip function

    let mut rip = StatelessHandler::new();
    rip.set_cmd_prefix("");
    for modal in rip::MODALS {
        rip.register_handle(modal, ripresp);
    }

    bot.add_handler(rip);

// Fck function

    let mut fck = StatelessHandler::new();
    fck.set_cmd_prefix("");
    for modal in fck::MODALS {
        fck.register_handle(modal, fckresp);
    }

    bot.add_handler(fck);

// Funkwhale function

    let mut fw = StatelessHandler::new();
    fw.set_cmd_prefix("%");
    fw.register_handle("artist", artresp);
    fw.register_handle("album", albresp);
    fw.register_handle("track", trackresp);

    bot.add_handler(fw);

// Choices function

    let mut choices = StatelessHandler::new();
    choices.set_cmd_prefix("");

    for modal in choices::MODALS {
        choices.register_handle(modal, yes_no);
    }

    bot.add_handler(choices);

// Login function

    let config = Settings::get_settings();
    let user = config.user;
    let password = config.password;
    let homeserver_url = config.homeserver_url;

    bot.run(&user, &password, &homeserver_url);
}
