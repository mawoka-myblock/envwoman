use crate::functions::helpers::{get_branch, get_data_from_proj};
use crate::{config, structs, ProjectFile};
use git2::Repository;
use regex::Regex;
use std::env;
use std::fs::File;
use std::path::PathBuf;

pub async fn main(
    name: Option<String>,
    from_file: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut current_path = env::current_dir()?;
    let repo: Option<Repository> = match Repository::open(&current_path) {
        Ok(repo) => Some(repo),
        Err(_) => None,
    };

    current_path.push(".envwoman.json");
    if current_path.exists() {
        println!("Project already exists. To delete it, please run \"envwoman delete\"");
        return Ok(());
    }
    let cfg: config::Config = confy::load("envwoman", None)?;
    let project_name: String = if name.is_none() {
        let temp = env::current_dir()?.to_str().unwrap().to_string();
        let re = Regex::new(r".*/(.*)").unwrap();
        let re_res = re.captures(&temp).unwrap();
        re_res
            .get(1)
            .map_or("".parse().unwrap(), |m| m.as_str().parse().unwrap())
    } else {
        name.unwrap().to_string()
    };

    let res = reqwest::Client::new()
        .get(
            "{api_url}/api/v1/projects/get/{project_name}"
                .replace("{api_url}", &cfg.api_url)
                .replace("{project_name}", &project_name),
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
            get_data_from_proj(&from_file, project.data.clone(), current_branch.clone()).await;
        if data.is_none() {
            println!("No data for current branch");
            return Ok(());
        }

        let copy_of_project_file = ProjectFile {
            name: project.name,
            description: match project.description {
                Some(description) => description,
                None => "".to_string(),
            },
            file: Some(from_file.clone()),
            environments: branches,
            selected_environment: current_branch,
        };
        let file = File::create(&current_path)?;
        serde_json::to_writer(file, &copy_of_project_file)?;
        println!("Successfully updated envs");
    } else {
        println!("Unknown error");
        return Err("Unknown error".into());
    }

    Ok(())
}
