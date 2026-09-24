use core::str;
use std::env;
use dotenvy::from_filename;
use serenity::futures::future::ok;

const config_file_name: &'static str = "config.env";

fn main() {
    from_filename(config_file_name).expect("File not found");
    let token_key: &'static str = "discord_token";
    let token: Result<String, env::VarError> = env::var(token_key);
    
    println!("Token: {:?}", token);
}
