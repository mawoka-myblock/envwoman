use magic_crypt::MagicCryptTrait;
use crate::{config, encryption, structs};
use crate::functions::helpers::get_data_from_proj;

pub async fn main(local: bool) -> Result<(), Box<dyn std::error::Error>> {
    let cfg: config::Config = confy::load("envwoman")?;
    let res = reqwest::Client::new()
        .get("{api_url}/api/v1/projects/list".replace("{api_url}", &cfg.api_url))
        .header("mawoka-auth-header", &cfg.api_key)
        .send()
        .await?;

    if res.status() == 401 {
        println!("Invalid API key");
        return Err("Invalid API key".into());
    } else if res.status() != 200 {
        return Err("Error: {}".replace("{}", &*res.status().to_string()).into());
    }
    let projects: structs::ListProjectResponse = serde_json::from_str(&res.text().await?).unwrap();
    let old_password = keyring::Entry::new("envwoman", "envwoman").get_password().unwrap();
    let new_password = dialoguer::Password::new().with_prompt("New password").with_confirmation("Confirm Password", "Passwords don't match").interact().unwrap();
    for project_list in projects {
        let res = reqwest::Client::new()
            .get("{api_url}/api/v1/projects/get/{project_name}"
                .replace("{api_url}", &cfg.api_url)
                .replace("{project_name}", &project_list.name))
            .header("mawoka-auth-header", &cfg.api_key)
            .send()
            .await?;
        if res.status() != 200 {
            return Err("Error: {}".replace("{}", &*res.status().to_string()).into());
        }
        let project: structs::ProjectResponse = serde_json::from_str(&res.text().await?).unwrap();
        for data in project.data {
            let mut return_data: Option<String> = None;
            for environment in data {
                if environment.contains_key(&current_branch) {
                    return_data = Some(encryption::decrypt_string(&environment[&current_branch]).ok()?);
                }
            }
        }
        let mc = new_magic_crypt!(old_password, 256);
        let res = mc.decrypt_base64_to_string(project.data)?;
    }
    Ok(())
}