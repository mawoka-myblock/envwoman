use crate::config;
use tokio::fs;

async fn local_logout() {
    let project_dir = confy::get_configuration_file_path("envwoman", None).unwrap();
    fs::remove_file(project_dir).await.unwrap();
}

pub async fn main(local: bool) -> Result<(), Box<dyn std::error::Error>> {
    let cfg: config::Config = confy::load("envwoman", None)?;
    return if local {
        local_logout().await;
        println!("Successfully logged out!");
        Ok(())
    } else {
        let input: String = dialoguer::Input::new()
            .with_prompt("Do you want to log every session out? Please enter \"YES\" to continue")
            .with_initial_text("No")
            .default("No".into())
            .interact_text()
            .unwrap();
        if input == "YES" {
            let res = reqwest::Client::new()
                .post("{api_url}/api/v1/users/logout".replace("{api_url}", &cfg.api_url))
                .header("mawoka-auth-header", &cfg.api_key)
                .send()
                .await?;
            if res.status().is_success() {
                local_logout().await;
                println!("Logged out all sessions, also this local one, so please log back in!");
            } else {
                println!("Failed to log out all sessions.");
                println!("{}", res.text().await?);
            }
            Ok(())
        } else {
            println!("{}", input);
            Ok(())
        }
    };
}
