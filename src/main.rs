extern crate config;
extern crate serde;
extern crate rand;
extern crate wana_kana;
extern crate ytr;
extern crate reqwest;
extern crate percent_encoding;
extern crate tokio;

mod funkwhale;
mod botconf;
mod choices;
mod simple;
mod language;

use matrix_bot_api::MatrixBot;
use matrix_bot_api::handlers::StatelessHandler;
use botconf::Settings;
use simple::{senddab, ripresp, fckresp, headpat, noticeme, versionresp, shruggie};
use choices::{yes_no, choose_resp, headstails};
use language::{kanaconvert, romaconvert};
use funkwhale::{artresp, trackresp, albresp};

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

//    let mut tran = StatelessHandler::new();
//    tran.set_cmd_prefix("%");
//    tran.register_handle("translate", translateme);

//    bot.add_handler(tran);

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
    for modal in simple::RIPMODALS {
        rip.register_handle(modal, ripresp);
    }

    bot.add_handler(rip);

// Fck function

    let mut fck = StatelessHandler::new();
    fck.set_cmd_prefix("");
    for modal in simple::FCKMODALS {
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

// Yes/No function

    let mut yesno = StatelessHandler::new();

    yesno.set_cmd_prefix("");

    for modal in choices::MODALS {
        yesno.register_handle(modal, yes_no);
    };

    bot.add_handler(yesno);

// Choices Function

    let mut choices = StatelessHandler::new();

    choices.set_cmd_prefix("%");
    choices.register_handle("choose", choose_resp);

    bot.add_handler(choices);

// Version Function

    let mut version = StatelessHandler::new();

    version.set_cmd_prefix("%");
    version.register_handle("version", versionresp);

    bot.add_handler(version);

// Shrug Function

    let mut shrug = StatelessHandler::new();

    shrug.set_cmd_prefix("%");
    shrug.register_handle("shrug", shruggie);

    bot.add_handler(shrug);

// Login function

    let config = Settings::get_settings();
    let user = config.user;
    let password = config.password;
    let homeserver_url = config.homeserver_url;

    bot.run(&user, &password, &homeserver_url);
}
