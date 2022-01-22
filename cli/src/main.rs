use std::path::{PathBuf};
mod functions;
use clap::{self, Parser};


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
    Push {
        #[clap(short, long)]
        no_pull: bool,
    },
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

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Command::parse();
    return match args {
        Command::Login => functions::login::main().await,
        Command::Pull => functions::pull::pull().await,
        Command::Push {no_pull} => functions::push::main(no_pull).await,
        Command::Init { name, from_file, description } => functions::init::init(name, from_file, description).await,
        Command::DeleteProject => functions::delete_project::delete_project().await,
        Command::Activate => activate().await,
        Command::Add => functions::add::main().await,
    };
}
