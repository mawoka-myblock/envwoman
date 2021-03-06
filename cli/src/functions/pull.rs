use crate::functions::helpers::{get_branch, get_data_from_proj};
use crate::structs::*;
use crate::{config, structs};
use git2::Repository;
use std::env;
use std::fs::File;

pub async fn pull(silent: bool) -> Result<(), Box<dyn std::error::Error>> {
    let cfg: config::Config = confy::load("envwoman", None)?;
    let mut config_file = env::current_dir()?;
    let repo: Option<Repository> = match Repository::open(&config_file) {
        Ok(repo) => Some(repo),
        Err(_) => None,
    };
    config_file.push(".envwoman.json");
    let file = File::open(&config_file)?;
    let project_file: ProjectFile = serde_json::from_reader(file)?;

    let old_project_file_file = project_file.file.clone();

    if project_file.file.is_none() {
        println!("Cannot update envs since the env-file is missing!");
        return Ok(());
    }

    let mut env_file = env::current_dir()?;
    env_file.push(&project_file.file.unwrap());
    let spinner = indicatif::ProgressBar::new_spinner();
    if !silent {
        spinner.set_message("Getting data from server...");
        spinner.enable_steady_tick(80);
    }

    let res = reqwest::Client::new()
        .get(
            "{api_url}/api/v1/projects/get/{project_name}"
                .replace("{api_url}", &cfg.api_url)
                .replace("{project_name}", &project_file.name),
        )
        .header("mawoka-auth-header", &cfg.api_key)
        .send()
        .await?;

    if res.status() == 404 {
        println!("Project not found");
        return Ok(());
    } else if res.status() == 401 {
        println!("Invalid API key");
        return Err("Invalid API key".into());
    } else if res.status() == 200 {
        let body = res.text().await?;
        let project: structs::ProjectResponse = match serde_json::from_str(&body) {
            Ok(project) => project,
            Err(_) => {
                return Err("Could not parse response from server".into());
            }
        };
        let temp_res = get_branch(repo).await;
        let current_branch = temp_res.0;
        let branches = temp_res.1;
        let data =
            get_data_from_proj(&env_file, project.data.clone(), current_branch.clone()).await;
        if data.is_none() {
            println!("No data for current branch");
            return Ok(());
        }

        // let mut env_file = env::current_dir()?;
        // env_file.push(&project_file.file.unwrap());
        let copy_of_project_file = ProjectFile {
            name: project.name,
            description: match project.description {
                Some(description) => description,
                None => "".to_string(),
            },
            file: old_project_file_file,
            environments: branches,
            selected_environment: current_branch,
        };
        // fs::remove_file(&config_file)?;

        let file = File::create(&config_file)?;
        serde_json::to_writer(file, &copy_of_project_file)?;
        if !silent {
            spinner.set_message("Successfully updated envs");
            spinner.finish()
        }
    } else {
        println!("Unknown error");
        return Err("Unknown error".into());
    }
    Ok(())
}
