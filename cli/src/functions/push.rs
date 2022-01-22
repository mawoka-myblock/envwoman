use std::{env, fs};
use std::collections::HashMap;
use std::fs::File;
use crate::{config, encryption, ProjectFile};

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: config::Config = confy::load("envwoman")?;
    let mut config_file = env::current_dir()?;
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

    let mut map = HashMap::new();
    map.insert("data", &data);
    map.insert("description", &project_file.description);


    let res = reqwest::Client::new()
        .post("{api_url}/api/v1/projects/update/{project_name}"
            .replace("{api_url}", &cfg.api_url)
            .replace("{project_name}", &project_file.name))
        .header("mawoka-auth-header", &cfg.api_key)
        .json(&map)
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
