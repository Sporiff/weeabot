extern crate serde;

use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub user: String,
    pub password: String,
    pub homeserver_url: String,
    pub funkwhale_url: String,
    pub translate: String,
    pub dab: String,
    pub rip: String,
    pub fck: String,
}

impl Settings {
    pub fn get_settings() -> Settings {
        let mut settings = config::Config::default();
        settings.merge(config::File::with_name("botconfig")).unwrap();
        Settings {
            user: settings.get_str("user").unwrap(),
            password: settings.get_str("password").unwrap(),
            homeserver_url: settings.get_str("homeserver_url").unwrap(),
            funkwhale_url: settings.get_str("funkwhale_url").unwrap(),
            translate: settings.get_str("translate").unwrap(),
            dab: settings.get_str("dab").unwrap(),
            rip: settings.get_str("rip").unwrap(),
            fck: settings.get_str("fck").unwrap(),
        }
    }
}