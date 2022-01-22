use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path};
use git2::{BranchType, Repository};
use regex::Regex;
use crate::encryption;


pub async fn get_branch(repo: Option<Repository>) -> (String, Vec<String>) {
    let current_branch: String;
    let mut branches: Vec<String> = Vec::new();
    if repo.is_some() {
        current_branch = Regex::new(r"refs/heads/(.*)").unwrap().captures(repo.as_ref().unwrap().head().unwrap().name().unwrap()).unwrap().get(1).unwrap().as_str().to_string();
        for branch in repo.unwrap().branches(Some(BranchType::Local)).unwrap() {
            branches.push(branch.unwrap().0.name().unwrap().map(String::from).unwrap());
        }
    } else {
        branches.push("standard".to_string());
        current_branch = "standard".to_string();
    }


    (current_branch, branches)
}

pub async fn get_data_from_proj(env_file: &Path, data: Vec<HashMap<String, String>>, current_branch: String) -> Option<String> {
    let mut return_data: Option<String> = None;
    for environment in data {
        if environment.contains_key(&current_branch) {
            return_data = Some(encryption::decrypt_string(&environment[&current_branch]).ok()?);
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(&env_file).ok()?;
            file.write_all(return_data.as_ref().unwrap().as_bytes()).ok()?;
            println!("Updated env-file: {}", &env_file.to_str().unwrap());
        }
    }

    return_data
}