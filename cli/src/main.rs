use serde_json;
use std::env;
use std::fs::File;
use std::path::{PathBuf};
use std::{fs};
use dotenv_parser::parse_dotenv;
mod functions;
use git2::{BranchType, Repository};
use clap::{self, Parser};
use tokio;


pub mod config;
pub mod encryption;
pub mod structs;

use crate::structs::ProjectFile;

#[macro_use]
extern crate magic_crypt;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
// #[structopt(name = "envwoman", about = "The official cli for the envwoman.")]
pub enum Command {
    #[clap(about = "Login into your account")]
    Login,
    #[clap(about = "Pull the changes from the server")]
    Pull,
    #[clap(about = "Push the changes to the server")]
    Push,
    #[clap(about = "Create a new project")]
    Init {
        #[clap(short, long)]
        name: Option<String>,
        #[clap(parse(from_os_str), short, long)]
        from_file: Option<PathBuf>,
        #[clap(short, long)]
        description: Option<String>,
    },
    /*#[clap(
    about = "Add an env-var to your project",
    override_help = "USAGE: SECRET_KEY my_secret_key"
    )]*/
    /*    Add {
            #[structopt(short, long)]
            key: String,
            #[structopt(short, long)]
            value: String,
        },*/
    #[clap(about = "Delete the project")]
    DeleteProject,
    Activate,
    Add,
}

/*async fn add(key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}*/


async fn activate() -> Result<(), Box<dyn std::error::Error>> {

    let mut config_file = env::current_dir()?;
    let repo = Repository::open(&config_file)?;
    // let branches: Vec<Option<&String>> = repo.branches(Some(BranchType::Local))?.into_iter().map(|b| b.unwrap().0.name().unwrap()).collect();
    let mut branches: Vec<Option<&str>> = Vec::new();
    for branch in repo.branches(Some(BranchType::Local))?.into_iter() {
        // let val = branch.unwrap().0.name().unwrap().to_owned();
        // let val = branch.unwrap();
        // branches.push(val.0.name().unwrap());
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
    let args = Command::parse();
    return match args {
        Command::Login => functions::login::main().await,
        Command::Pull => functions::pull::pull().await,
        Command::Push => functions::push::main().await,
        Command::Init { name, from_file, description } => functions::init::init(name, from_file, description).await,
        Command::DeleteProject => functions::delete_project::delete_project().await,
        Command::Activate => activate().await,
        Command::Add => functions::add::main().await,
    };
}
