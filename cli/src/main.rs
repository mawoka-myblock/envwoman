use std::path::{PathBuf};

mod functions;

use clap::{self, Parser};


pub mod config;
pub mod encryption;
pub mod structs;

use crate::structs::ProjectFile;
use color_eyre::eyre::Result;


#[macro_use]
extern crate magic_crypt;
#[macro_use] extern crate prettytable;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
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
    DeleteProject {
        #[clap(short, long)]
        force: bool,
        #[clap(short, long)]
        name: Option<String>,
    },
    Activate,
    Add,
    Reinit {
        #[clap(short, long)]
        name: Option<String>,
        #[clap(short, long)]
        file: PathBuf,
    },
    ListProjects,
/*
    ChangePassword {
        #[clap(short, long)]
        local: bool,
    }
    */
}

/*async fn add(key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}*/


async fn activate() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: config::Config = confy::load("envwoman")?;
    let _guard: sentry::ClientInitGuard;
    if cfg.sentry_enabled {
        _guard = sentry::init(("https://b8a0e0246043409092a000cc3afbb6fb@o661934.ingest.sentry.io/6162750", sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 1.0,
            ..Default::default()
        }));
    }
    if !cfg.trace_enabled {
        std::env::set_var("RUST_SPANTRACE", "0");
    }
    color_eyre::install()?;
    let args = Command::parse();
    return match args {
        Command::Login => functions::login::main().await,
        Command::Pull => functions::pull::pull(false).await,
        Command::Push { no_pull } => functions::push::main(no_pull).await,
        Command::Init { name, from_file, description } => functions::init::init(name, from_file, description).await,
        Command::DeleteProject { force, name } => functions::delete_project::delete_project(force, name).await,
        Command::Activate => activate().await,
        Command::Add => functions::add::main().await,
        Command::Reinit {name, file} => functions::reinit::main(name, file).await,
        Command::ListProjects => functions::list_projects::main().await,
        // Command::ChangePassword {local} => functions::change_password::main(local).await,
    };
}
