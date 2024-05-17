
mod generators;

use std::time::Duration;
use colored::*;
use clap::{Command};
use dialoguer::{Select};
use indicatif::{ProgressBar, ProgressStyle};
use crate::generators::ci_cd::ci_cd::{CiCd, GitTarget};
use crate::generators::config::config::Config;
use crate::generators::docker::docker::Docker;
use crate::generators::docker::docker_item::DockerItem;
use crate::generators::git::github::Github;
use crate::generators::javascript::frameworks::nest::Nest;
use crate::generators::javascript::package_manager::PackageManager;
use crate::generators::progress_bar::start_progress_bar;

fn main() {

    let matches = Command::new("unmake cli workspace")
        .version("1.0")
        .author("svallotale@gmail.com")
        .about("CLI interface for convenient project development")
        .subcommand(
            Command::new("new")
                .about("Create a new project")
        )
        .subcommand(
            Command::new("add")
                .about("Add modules/dependencies to the project")
        )
        .get_matches();

    if let Some(("new", _)) = matches.subcommand() {
        let project_name = dialoguer::Input::<String>::new()
            .with_prompt("Project name".cyan().bold().to_string())
            .interact_text()
            .unwrap();
        
        let items = vec![
            "Nest",
            "Next",
            "Django",
            "Fastapi",
        ];
        let selection = Select::new()
            .with_prompt("What do you choose?".bold().cyan().to_string())
            .items(&items)
            .default(0)
            .interact()
            .unwrap();

        match items[selection] {
            "Nest" => {
                let package_manager = PackageManager::how_package_manager();

                let docker_items = Docker::how_docker_use();
                
                let progress_bar = start_progress_bar();
                progress_bar.set_message("Creating new Nestjs project...");
                
                let nestjs_app = Nest::new(package_manager);
                nestjs_app.create(project_name);

                progress_bar.finish_with_message("Done");
                
                // let config = Config::new(
                //     Option::Some(Docker::new(
                //         "Dockerfile".to_string(),
                //         "compose.yml".to_string(),
                //         vec![
                //             DockerItem::new(
                //                 "postgresql".to_string(),
                //                 "postgres:16-alpine".to_string(),
                //                 vec![
                //                     "postgres_data:/var/lib/postgresql/data".to_string(),
                //                 ],
                //                 vec![
                //                     "POSTGRES_USER: svallotale".to_string(),
                //                     "POSTGRES_PASSWORD: svallotale".to_string(),
                //                     "POSTGRES_DB: database".to_string(),
                //                 ],
                //                 vec![
                //                     "5432:5432".to_string()
                //                 ],
                //                 vec![]
                //             )
                //         ]
                //     )),
                //     Option::Some(CiCd::new(
                //         GitTarget::Github(Github {
                //             remote_path: "test.url".to_string(),
                //         }),
                //     )),
                // );
                // config.save_to_file().expect("Error saved");
            },
            "Next" => println!("You chose: {}", items[selection]),
            "Django" => println!("You chose: {}", items[selection]),
            "Fastapi" => println!("You chose: {}", items[selection]),
            _ => println!("You chose: {}", items[selection]),
        }
    }

    if let Some(("add", sub_matches)) = matches.subcommand() {
        let items = vec![
            "Dockerhub",
            "Modules",
        ];
        let selection = Select::new()
            .with_prompt("What do you choose?".bold().cyan().to_string())
            .items(&items)
            .default(0)
            .interact()
            .unwrap();

    }
}
