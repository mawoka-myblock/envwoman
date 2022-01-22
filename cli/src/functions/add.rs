use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::Write;
use git2::{BranchType, Repository};
use regex::Regex;
use crate::{config, encryption, ProjectFile, structs};

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: config::Config = confy::load("envwoman")?;
    let mut config_file = env::current_dir()?;
    let repo: Option<Repository> = match Repository::open(&config_file) {
        Ok(repo) => Some(repo),
        Err(_) => None
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

    let res = reqwest::Client::new()
        .get("{api_url}/api/v1/projects/get/{project_name}"
            .replace("{api_url}", &cfg.api_url)
            .replace("{project_name}", &project_file.name))
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
                println!("Could not parse json into struct");
                return Ok(());
            }
        };
        let mut data: Option<String> = None;
        let mut branches: Vec<String> = Vec::new();
        let current_branch: String;
        if repo.is_some() {
            current_branch = Regex::new(r"refs/heads/(.*)").unwrap().captures(repo.as_ref().unwrap().head().unwrap().name().unwrap()).unwrap().get(1).unwrap().as_str().to_string();
            for branch in repo.unwrap().branches(Some(BranchType::Local))? {
                branches.push(branch.unwrap().0.name().unwrap().map(String::from).unwrap());
            }
            println!("Current branch: {}, All branches available: {:?}", &current_branch, branches);
        } else {
            branches.push("standard".to_string());
            current_branch = "standard".to_string();
        }
        for environment in &project.data {
            if environment.contains_key(&current_branch) {
                data = Some(encryption::decrypt_string(&environment[&current_branch])?);
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&env_file)?;
                file.write_all(data.as_ref().unwrap().as_bytes())?;
                println!("Updated env-file: {}", &env_file.to_str().unwrap());
            }
        }
        if data.is_none() {
            println!("No data for current branch");
            return Ok(());
        }
        // let mut env_file = env::current_dir()?;
        // env_file.push(&project_file.file.unwrap());
        File::create(&env_file)?;
        fs::remove_file(&env_file)?;
        File::create(&env_file)?;
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&env_file)
            .unwrap();
        file.write_all(data.unwrap().as_bytes())?;
        println!("Successfully updated envs");
        let mut new_vec = Vec::new();
        new_vec.push("lol".to_string());
        let copy_of_project_file = ProjectFile {
            name: project.name,
            description: match project.description {
                Some(description) => description,
                None => "".to_string(),
            },
            file: old_project_file_file,
            environments: new_vec,
            selected_environment: "standard".to_string(),
        };
        // fs::remove_file(&config_file)?;

        let file = File::create(&config_file)?;
        println!("{:?}", &copy_of_project_file);
        serde_json::to_writer(file, &copy_of_project_file)?;
        println!("Successfully updated envs");
    } else {
        println!("Unknown error");
        return Err("Unknown error".into());
    }
    Ok(())
}