use std::{env, fs};
use std::collections::HashMap;
use std::fs::File;
use git2::{BranchType, Repository};
use regex::Regex;
use crate::{config, encryption, ProjectFile};
use crate::functions::pull::pull;
use crate::structs::UpdateProject;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
//    pull().await?;
    let cfg: config::Config = confy::load("envwoman")?;
    let mut config_file = env::current_dir()?;
    let repo: Option<Repository> = match Repository::open(&config_file) {
        Ok(repo) => Some(repo),
        Err(_) => None
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


    let current_branch: String;
    let mut branches: Vec<String> = Vec::new();
    if repo.is_some() {
        current_branch = Regex::new(r"refs/heads/(.*)").unwrap().captures(repo.as_ref().unwrap().head().unwrap().name().unwrap()).unwrap().get(1).unwrap().as_str().to_string();
        for branch in repo.unwrap().branches(Some(BranchType::Local))? {
            branches.push(branch.unwrap().0.name().unwrap().map(String::from).unwrap());
        }
    } else {
        branches.push("standard".to_string());
        current_branch = "standard".to_string();
    }

    let update_project: UpdateProject = UpdateProject {
        members: None,
        environments: Option::from(branches),
        selected_environment: Option::from(current_branch),
        description: None,
        data: Option::from(data)
    };

    println!("{:?}", &update_project);
    let res = reqwest::Client::new()
        .post("{api_url}/api/v1/projects/update/{project_name}"
            .replace("{api_url}", &cfg.api_url)
            .replace("{project_name}", &project_file.name))
        // .post("https://bin.muetsch.io/4tt5qi0")
        .header("mawoka-auth-header", &cfg.api_key)
        .body(match serde_json::to_string(&update_project) {
            Ok(body) => body,
            Err(err) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, err)))
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
        println!("Unknown error");
        return Err("Unknown error".into());
    }


    Ok(())
}
