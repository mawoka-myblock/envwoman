use std::io;
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::config;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: config::Config = confy::load("envwoman", None)?;
    if !cfg.api_key.is_empty() {
        println!("Already logged in");
        return Ok(());
    }
    println!("To login, I'll try to open the browser for you, so you can easily log in. \n Please paste the code you'll get right in here!");
    // if webbrowser::open(&"{}/login".replace("{}", &cfg.api_url)).is_err() {
    if webbrowser::open("https://envwoman.mawoka.eu/login").is_err() {
        println!(
            "Could not open browser, please open this URL manually: https://envwoman.eu.org/login"
        );
    }
    println!("\n");
    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();
    let code = code.trim();
    println!("\n {}", code);
    if code.len() != 10 {
        println!("Please enter a valid code");
        return Err("Invalid code".into());
    }
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Logging in...");
    spinner.enable_steady_tick(80);
    let resp = reqwest::Client::new()
        // .get("https://envwoman.eu.org/api/v1/cli-login/token/{}"
        .get(
            "{api_url}/api/v1/cli-login/token/{token}"
                .replace("{token}", code)
                .replace("{api_url}", &cfg.api_url),
        )
        .send()
        .await?;
    let password: String;
    if resp.status() == 404 {
        println!("Invalid code");
        return Err("Invalid code".into());
    } else if resp.status() == 200 {
        spinner.set_message("Successfully logged in!");
        spinner.finish();
        let body = resp.text().await?;
        let mut modified_cfg = cfg;
        password = dialoguer::Password::new().with_prompt("Please enter an encryption-password for this project")
            .with_confirmation("Confirm password", "Passwords mismatching")
            .interact().unwrap();
        modified_cfg.api_key = body;
        modified_cfg.salt = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        confy::store("envwoman", None, &modified_cfg)?;
        println!("Logged in successfully!");

    } else {
        println!("Unknown error");
        return Err("Unknown error".into());
    }


    let entry = keyring::Entry::new("envwoman", "envwoman");
    entry.set_password(&password)?;

    Ok(())
}
