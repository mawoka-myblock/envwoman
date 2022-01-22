use std::{env, fs};
use std::fs::File;
use dialoguer::Confirm;
use crate::{config, ProjectFile};

pub async fn delete_project() -> Result<(), Box<dyn std::error::Error>> {
    let mut current_path = env::current_dir()?;
    current_path.push(".envwoman.json");
    if !current_path.exists() {
        println!("Project does not exist");
        return Ok(());
    }
    let file = File::open(&current_path)?;
    let project_config: ProjectFile = serde_json::from_reader(file)?;
    let cfg: config::Config = confy::load("envwoman")?;
    if cfg.api_key.is_empty() {
        println!("You are not logged in. Run \"envwoman login\" to log in");
        return Ok(());
    }
    let resp = reqwest::Client::new()
        // .get("https://envwoman.eu.org/api/v1/cli-login/token/{}"
        .delete(
            "{api_url}/api/v1/projects/delete/{project}"
                .replace("{project}", &project_config.name)
                .replace("{api_url}", &cfg.api_url),
        )
        .header(
            "mawoka-auth-header",
            &cfg.api_key,
        )
        .send()
        .await?;
    if resp.status() == 200 {
        println!("Deleted project successfully!");
        fs::remove_file(current_path)?;
    } else {
        println!("The project doesn't exist on the server. Shall I delete the local file?");
        if Confirm::new()
            .with_prompt("Delete local file?")
            .interact()?
        {
            fs::remove_file(current_path)?;
        }
    }
    Ok(())
}
