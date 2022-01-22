use std::{env, fs, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use dialoguer::Confirm;
use dotenv_parser::parse_dotenv;
use regex::Regex;
use crate::{config, encryption};
use crate::structs::ProjectFile;

pub async fn init(
    name: Option<String>,
    from_file: Option<PathBuf>,
    description: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let cfg: config::Config = confy::load("envwoman")?;
    let project_name: String;
    let mut current_path = env::current_dir()?;
    current_path.push(".envwoman.json");
    if current_path.exists() {
        println!("Project already exists. To delete it, please run \"envwoman delete\"");
        return Ok(());
    }

    if name.is_none() {
        let temp = env::current_dir()?.to_str().unwrap().to_string();
        let re = Regex::new(r".*/(.*)").unwrap();
        let re_res = re.captures(&temp).unwrap();
        project_name = re_res
            .get(1)
            .map_or("".parse().unwrap(), |m| m.as_str().parse().unwrap());
    } else {
        project_name = name.unwrap().to_string();
    }

    if Confirm::new()
        .with_prompt(
            "Do you want to create a new project called \"{}\"".replace("{}", &project_name),
        )
        .interact()?
    {
    } else {
        println!("If you want to choose a custom name, use envwoman init \"your_project_name\"");
        return Ok(());
    }
    let description_new: String;
    if description.is_some() {
        description_new = description.unwrap().to_string();
    } else {
        if Confirm::new().with_prompt("Do you want to add a description to your project?").interact()? {
            let mut buffer = String::new();
            let mut stdin = io::stdin();
            stdin.read_line(&mut buffer)?;
            description_new = buffer.trim().to_string();
        } else { description_new = "".to_string(); }
    }


    let config_data: ProjectFile;
    let mut env_file = env::current_dir()?;
    env_file.push(".env");
    let current_env: Option<String>;
    let mut new_vec = Vec::new();
    new_vec.push("lol".to_string());
    if from_file.is_none() || !env_file.exists() {
        config_data = ProjectFile {
            name: project_name,
            file: None,
            description: description_new,
            environments: new_vec,
            selected_environment: "standard".to_string(),
        };
        current_env = None;
    } else {
        let mut new_vec = Vec::new();
        new_vec.push("lol".to_string());
        config_data = ProjectFile {
            name: project_name,
            file: Some(from_file.unwrap()),
            description: description_new,
            environments: new_vec,
            selected_environment: "standard".to_string(),
        };
        let mut file = File::open(&env_file)?;
        let mut read_file = String::new();
        file.read_to_string(&mut read_file)?;
        let parsed_env = parse_dotenv(&read_file).unwrap();
        let env_data_vec: HashMap<&String, &String> = HashMap::from_iter(parsed_env.iter());
        // let end_data_json = serde_json::from_str(&)?;
        let test_str = "MOIN!".to_string();
        // current_env = Some(encryption::encrypt_string(&Some(serde_json::from_str(&String::from_utf8_lossy(&env_data_vec.as_bytes())).unwrap()).unwrap())?);
        // current_env = Some(encryption::encrypt_string(&Some(serde_json::from_slice(&env_data_vec.as_slice()).unwrap()).unwrap())?);
        let env_data_str: serde_json::value::Value = serde_json::from_str(&format!("{:?}", env_data_vec))?;
        current_env = Some(encryption::encrypt_string(&Some(env_data_str.to_string()).unwrap())?);
    }
    let mut map = HashMap::new();
    map.insert("name", &config_data.name);
    map.insert("description", &config_data.description);

    let current_env_new: String;
    if current_env.is_none() {
        current_env_new = "".to_string();
    } else {
        current_env_new = current_env.unwrap();
    }
    map.insert("data", &current_env_new);

    println!("Creating project on the server...");
    let resp = reqwest::Client::new()
        // .get("https://envwoman.eu.org/api/v1/cli-login/token/{}"
        .post(
            "{api_url}/api/v1/projects/create"
                .replace("{api_url}", &cfg.api_url),
        )
        .json(&map)
        .header(
            "mawoka-auth-header",
            &cfg.api_key,
        )
        .send()
        .await?;
    return if resp.status() == 200 {
        println!("Project created successfully!");
        // println!("Project created successfully!");
        File::create(&current_path)?.write_all(serde_json::to_string(&config_data)?.as_bytes())?;
        Ok(())
    } else if resp.status() == 409 {
        println!("A project with this name already exists!");
        if current_path.exists() {
            fs::remove_file(&current_path)?;
        }
        Ok(())
    } else {
        println!("Something went wrong!");
        if current_path.exists() {
            fs::remove_file(&current_path)?;
        }
        Ok(())
    }


}