use crate::{config, ProjectFile};
use dialoguer::Confirm;
use indicatif::ProgressBar;
use reqwest::Response;
use std::fs::File;
use tokio::fs;
use std::{env};

async fn delete_req(
    project_name: String,
    api_url: String,
    api_key: String,
) -> Result<Response, reqwest::Error> {
    let resp = reqwest::Client::new()
        // .get("https://envwoman.eu.org/api/v1/cli-login/token/{}"
        .delete(
            "{api_url}/api/v1/projects/delete/{project}"
                .replace("{project}", &project_name)
                .replace("{api_url}", &api_url),
        )
        .header("mawoka-auth-header", &api_key)
        .send()
        .await;

    resp
}

pub async fn delete_project(
    force: bool,
    name: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut current_path = env::current_dir()?;
    current_path.push(".envwoman.json");
    if !current_path.exists() && name.is_none() {
        println!("Project does not exist");
        return Ok(());
    }
    let cfg: config::Config = confy::load("envwoman", None)?;
    if name.as_ref().is_some() {
        if !force {
            #[warn(clippy::collapsible_if)]
            if !Confirm::new()
                .with_prompt(format!(
                    "Do you want to continue to delete \"{}\"",
                    name.as_ref().unwrap()
                ))
                .interact()?
            {
                return Ok(());
            }
        }
        let progress_bar = ProgressBar::new_spinner();
        progress_bar.enable_steady_tick(80);
        progress_bar.set_message(format!(
            "Trying to delete the project \"{}\" from the server...",
            name.as_ref().unwrap()
        ));
        let resp = delete_req(
            name.as_ref().unwrap().to_string(),
            cfg.api_url.clone(),
            cfg.api_key.clone(),
        )
        .await?;
        if resp.status().is_success() {
            progress_bar.set_message(format!(
                "Project \"{}\" deleted successfully",
                name.as_ref().unwrap()
            ));
        } else {
            progress_bar.set_message(format!(
                "Project \"{}\" could not be deleted",
                name.as_ref().unwrap()
            ));
        }
        progress_bar.finish();
        if current_path.exists()
            && Confirm::new()
                .with_prompt("Delete local file?")
                .interact()?
        {
            fs::remove_file(&current_path).await?;
        }
        return Ok(());
    }
    let file = File::open(&current_path)?;
    let project_config: ProjectFile = serde_json::from_reader(file)?;

    if cfg.api_key.is_empty() {
        println!("You are not logged in. Run \"envwoman login\" to log in");
        return Ok(());
    }
    if !force {
        #[warn(clippy::collapsible_if)]
        if !Confirm::new()
            .with_prompt(format!(
                "Do you want to continue to delete \"{}\"",
                &project_config.name
            ))
            .interact()?
        {
            return Ok(());
        }
    }
    let resp = delete_req(
        project_config.name.clone(),
        cfg.api_url.clone(),
        cfg.api_key.clone(),
    )
    .await?;
    if resp.status() == 200 {
        println!("Deleted project successfully!");

        if name.as_ref().is_none() {
            fs::remove_file(&current_path).await?;
        }
    } else {
        println!("The project doesn't exist on the server. Shall I delete the local file?");
        if Confirm::new()
            .with_prompt("Delete local file?")
            .interact()?
        {
            fs::remove_file(current_path).await?;
        }
    }
    Ok(())
}
