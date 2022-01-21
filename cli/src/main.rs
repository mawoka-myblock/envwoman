use confy;
use dialoguer::Confirm;
use keyring;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf};
use std::{fs, io};
use std::borrow::Borrow;
use std::collections::HashMap;
use dotenv_parser::parse_dotenv;
use structopt::StructOpt;
use std::fs::OpenOptions;
use webbrowser;
use git2::{BranchType, Repository};

pub mod config;
pub mod encryption;
mod structs;

use rand::{distributions::Alphanumeric, Rng};
use rpassword;

#[macro_use]
extern crate magic_crypt;


#[derive(Serialize, Deserialize, Debug)]
struct ProjectFile {
    name: String,
    file: Option<PathBuf>,
    description: String,
    environments: Vec<String>,
    selected_environment: String,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "envwoman", about = "The official cli for the envwoman.")]
enum Command {
    Login,
    Pull,
    Push,
    Init {
        #[structopt(short, long)]
        name: Option<String>,
        #[structopt(parse(from_os_str), short, long)]
        from_file: Option<PathBuf>,
        #[structopt(short, long)]
        description: Option<String>,
    },
    #[structopt(
    about = "Add an env-var to your project",
    help = "USAGE: SECRET_KEY my_secret_key"
    )]
    /*    Add {
            #[structopt(short, long)]
            key: String,
            #[structopt(short, long)]
            value: String,
        },*/
    DeleteProject,
    Activate,
}

async fn login() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: config::Config = confy::load("envwoman")?;
    if !cfg.api_key.is_empty() {
        println!("Already logged in");
        return Ok(());
    }
    println!("To login, I'll try to open the browser for you, so you can easily log in. \n Please paste the code you'll get right in here!");
    if webbrowser::open(&"{}/login".replace("{}", &cfg.api_url)).is_err() {
        println!(
            "Could not open browser, please open this URL manually: https://envwoman.eu.org/login"
        );
    }
    println!("\n");
    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();
    let code = code.trim();
    println!("\n {}", code);
    if code.len() != 10 {
        println!("Please enter a valid code");
        return Err("Invalid code".into());
    }

    let resp = reqwest::Client::new()
        // .get("https://envwoman.eu.org/api/v1/cli-login/token/{}"
        .get(
            "{api_url}/api/v1/cli-login/token/{token}"
                .replace("{token}", code)
                .replace("{api_url}", &cfg.api_url),
        )
        .send()
        .await?;
    if resp.status() == 404 {
        println!("Invalid code");
        return Err("Invalid code".into());
    } else if resp.status() == 200 {
        let body = resp.text().await?;
        println!("{}", body);
        let mut modified_cfg = cfg;
        modified_cfg.api_key = body.into();
        modified_cfg.salt = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        confy::store("envwoman", &modified_cfg)?;
        println!("Successfully logged in!");
    } else {
        println!("Unknown error");
        return Err("Unknown error".into());
    }
    let cfg: config::Config = confy::load("envwoman")?;
    let service = "envwoman";
    let username = &cfg.api_key;
    println!("Please enter an encryption-password for this project");
    let password = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    let password_confirm = rpassword::read_password_from_tty(Some("Confirm password: ")).unwrap();
    if password != password_confirm {
        println!("Passwords do not match");
        return Ok(());
    }
    let entry = keyring::Entry::new(&service, &username);
    entry.set_password(&password)?;

    Ok(())
}

async fn pull() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: config::Config = confy::load("envwoman")?;
    let mut config_file = env::current_dir()?;
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
        let data = encryption::decrypt_string(&project.data);
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

async fn push() -> Result<(), Box<dyn std::error::Error>> {
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

async fn init(
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

/*async fn add(key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}*/

async fn delete_project() -> Result<(), Box<dyn std::error::Error>> {
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

async fn activate() -> Result<(), Box<dyn std::error::Error>> {

    let mut config_file = env::current_dir()?;
    let repo = Repository::open(&config_file)?;
    // let branches: Vec<Option<&String>> = repo.branches(Some(BranchType::Local))?.into_iter().map(|b| b.unwrap().0.name().unwrap()).collect();
    let mut branches: Vec<Option<&str>> = Vec::new();
    for branch in repo.branches(Some(BranchType::Local))?.into_iter() {
        // let val = branch.unwrap().0.name().unwrap().to_owned();
        let val = branch.unwrap();
        branches.push(val.0.name().unwrap());
        let test: String = Some("lol".to_string()).unwrap();

    }
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

    let dotenv_map = parse_dotenv(&file_content).unwrap();
    /*
        for (key, value) in dotenv_map.iter() {
            println!("{}={}", key, value);
            // env::set_var(key, value);
        }
        */
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Command::from_args();
    return match args {
        Command::Login => login().await,
        Command::Pull => pull().await,
        Command::Push => push().await,
        Command::Init { name, from_file, description } => init(name, from_file, description).await,
        Command::DeleteProject => delete_project().await,
        Command::Activate => activate().await,
    };
}
