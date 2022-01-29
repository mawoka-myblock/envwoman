use crate::functions::{helpers::get_branch, pull::pull};
use crate::structs::UpdateProject;
use crate::{config, encryption, ProjectFile};
use git2::Repository;
use std::fs::File;
use std::{env, fs};

pub async fn main(no_pull: bool) -> Result<(), Box<dyn std::error::Error>> {
    if !no_pull {
        pull(true).await?;
    }
    let cfg: config::Config = confy::load("envwoman", None)?;
    let mut config_file = env::current_dir()?;
    let repo: Option<Repository> = match Repository::open(&config_file) {
        Ok(repo) => Some(repo),
        Err(_) => None,
    };
    config_file.push(".envwoman.json");
    let file = File::open(&config_file)?;
    let project_file: ProjectFile = serde_json::from_reader(file)?;

    if project_file.file.is_none() {
        println!("Cannot update envs since the env-file is missing!");
        return Ok(());
    }

    let mut env_file = env::current_dir()?;
    env_file.push(&project_file.file.unwrap());
    let file_content = fs::read_to_string(&env_file)?;
    let data = encryption::encrypt_string(&file_content)?;

    let temp_res = get_branch(repo).await;
    let current_branch = temp_res.0;
    let branches = temp_res.1;

    let update_project: UpdateProject = UpdateProject {
        members: None,
        environments: Option::from(branches),
        selected_environment: Option::from(current_branch),
        description: None,
        data: Option::from(data),
    };
    // println!("{:?}", encryption::decrypt_string(&update_project.data.as_ref().unwrap()));
    let res = reqwest::Client::new()
        .post(
            "{api_url}/api/v1/projects/update/{project_name}"
                .replace("{api_url}", &cfg.api_url)
                .replace("{project_name}", &project_file.name),
        )
        .header("mawoka-auth-header", &cfg.api_key)
        .body(match serde_json::to_string(&update_project) {
            Ok(body) => body,
            Err(err) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    err,
                )))
            }
        })
        .send()
        .await?;

    if res.status() == 404 {
        println!("Project not found");
        return Ok(());
    } else if res.status() == 401 {
        println!("Invalid API key");
        return Err("Invalid API key".into());
    } else if res.status() == 200 {
        println!("Successfully updated envs");
    } else {
        return Err("Unknown error".into());
    }

    Ok(())
}
