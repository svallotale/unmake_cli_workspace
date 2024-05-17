use std::{env, fs};
use std::io::Write;
use colored::Colorize;
use dialoguer::{Confirm, Select};
use serde::{Deserialize, Serialize};
use crate::generators::config::config::Config;
use crate::generators::docker::docker_item::DockerItem;

#[derive(Serialize, Deserialize)]
pub struct Docker {
    pub dockerfile_path: String,
    pub compose_path: String,
    pub services: Vec<DockerItem>,
}

impl Docker {

    pub fn new(dockerfile_path: String, compose_path: String, services: Vec<DockerItem>) -> Docker {
        Docker {
            dockerfile_path,
            compose_path,
            services,
        }
    }

    pub fn how_docker_use() {

        let confirmation_use = Confirm::new()
            .with_prompt("Add docker to the preoct".cyan().bold().to_string())
            .default(true)
            .interact()
            .unwrap();

        if confirmation_use {
            let items = DockerItem::get_template_names();
            let selection = Select::new()
                .with_prompt("What do you choose?".bold().cyan().to_string())
                .items(&items)
                .interact()
                .unwrap();

        }


    }


    pub fn generate_compose_content(&self) -> String {
        let mut compose_content = String::new();
        compose_content.push_str("\nservices:\n\n");

        for service in &self.services {
            compose_content.push_str(&format!("  {}:\n", service.name));
            compose_content.push_str(&format!("    container_name: \"${{PROJECT_ALIAS}}_{}\"\n", service.name));
            compose_content.push_str(&format!("    image: {}\n", service.image));
            compose_content.push_str("    restart: unless-stopped\n");
            compose_content.push_str("    ports:\n");
            for port in &service.ports {
                compose_content.push_str(&format!("      - \"{}\"\n", port));
            }
            if !service.environment.is_empty() {
                compose_content.push_str("    environment:\n");
                for env_var in &service.environment {
                    compose_content.push_str(&format!("      {}\n", env_var));
                }
            }
            if !service.volumes.is_empty() {
                compose_content.push_str("    volumes:\n");
                for volume in &service.volumes {
                    compose_content.push_str(&format!("      - \"{}\"\n", volume));
                }
            }
            if !service.command.is_empty() {
                compose_content.push_str(&format!("    command: {:?}\n", service.command));
            }
        }

        if !&self.services.is_empty() {
            compose_content.push('\n');
            compose_content.push_str("volumes:\n");
            for service in &self.services {
                for volume in &service.volumes {
                    if let Some(volume_name) = volume.split(':').next()  {
                        if !volume.starts_with("./") {
                            compose_content.push_str(&format!("  {}:\n", volume_name))
                        }
                    }
                }
            }
        }

        compose_content
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let current_dir = env::current_dir()?;
        let config_path = current_dir.join(&self.compose_path);

        let compose_content = self.generate_compose_content();
        let mut file = fs::File::create(&config_path)?;
        file.write_all(compose_content.as_bytes())?;

        Ok(())
    }


}
