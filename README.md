# weeabot

A Matrix bot written in Rust

## Features

This bot is designed to interact with a chatroom using
a host of functions ranging from helpful language tricks
and fun reactions to various triggers set up in the
`BotConfig.toml` file.

## Triggers

`%senpai` - Triggers a "Notice me senpai" message
`%headpat` - Triggers a "Nyan nyan" message
`%kana <string>` - Converts given string into Japanese kana
`%roma <string>` - Converts given string into roman characters
`%translate <language code> <string>` - Translates given
string into the language given in the language code. e.g.:

```
%translate fr hello

bonjour
```

The bot also responds to the following words at a frequency
determined in the `BotConfig.toml` file:

`dab` - Responds with a dabbing kaomoji
`rip` - Responds with a sad "RIP" message
`fuck` - Responds with an angry "Fuck" message

This bot makes use of the
[matrix_bot_api crate](https://docs.rs/matrix_bot_api/0.4.0/matrix_bot_api/)
