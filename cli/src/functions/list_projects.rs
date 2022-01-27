use crate::{config, structs};
use prettytable::{Table, Row, Cell};

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let body = res.text().await?;
    let projects: structs::ListProjectResponse = serde_json::from_str(&body).unwrap();
    let mut table = Table::new();
    table.add_row(row!["Name", "Description", "Created at", "Updated at", "Environments"]);
    for project in projects {
        table.add_row(Row::new(vec![
            Cell::new(&project.name),
            Cell::new(&project.description),
            Cell::new(&project.date_created.to_string()),
            Cell::new(&project.date_modified.to_string()),
            Cell::new(&project.environments.join(", ")),
        ]));
    }
    table.printstd();
    Ok(())
}