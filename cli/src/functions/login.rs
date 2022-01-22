use std::io;
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::config;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: config::Config = confy::load("envwoman")?;
    if !cfg.api_key.is_empty() {
        println!("Already logged in");
        return Ok(());
    }
    println!("To login, I'll try to open the browser for you, so you can easily log in. \n Please paste the code you'll get right in here!");
    if webbrowser::open(&"{}/login".replace("{}", &cfg.api_url)).is_err() {
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

    let resp = reqwest::Client::new()
        // .get("https://envwoman.eu.org/api/v1/cli-login/token/{}"
        .get(
            "{api_url}/api/v1/cli-login/token/{token}"
                .replace("{token}", code)
                .replace("{api_url}", &cfg.api_url),
        )
        .send()
        .await?;
    if resp.status() == 404 {
        println!("Invalid code");
        return Err("Invalid code".into());
    } else if resp.status() == 200 {
        let body = resp.text().await?;
        println!("{}", body);
        let mut modified_cfg = cfg;
        modified_cfg.api_key = body;
        modified_cfg.salt = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        confy::store("envwoman", &modified_cfg)?;
        println!("Successfully logged in!");
    } else {
        println!("Unknown error");
        return Err("Unknown error".into());
    }
    let cfg: config::Config = confy::load("envwoman")?;
    let service = "envwoman";
    let username = &cfg.api_key;
    println!("Please enter an encryption-password for this project");
    let password = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    let password_confirm = rpassword::read_password_from_tty(Some("Confirm password: ")).unwrap();
    if password != password_confirm {
        println!("Passwords do not match");
        return Ok(());
    }
    let entry = keyring::Entry::new(service, username);
    entry.set_password(&password)?;

    Ok(())
}
