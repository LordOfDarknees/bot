use core::str;
use std::{env::{self, VarError}};
use dotenvy::from_filename;
use serenity::futures::future::ok;

// const config_file_name: &'static str = "config.env";

fn get_token() -> Result<String, env::VarError> {

    let config_file_name: &str = "config.env";

    from_filename(config_file_name).expect("File not found");
    let token_key: &str = "discord_token";
    let token: Result<String, env::VarError> = env::var(token_key);
    return token;
}

fn main() {
    let retrieved_token: Result<String, VarError> = get_token();
    match retrieved_token {
        Ok(v) => println!("Token: {:?}", v),
        Err(e) => panic!("Token not found. {:?}", e)
    }
}
