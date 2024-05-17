use std::{env, fs};
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use crate::generators::ci_cd::ci_cd::CiCd;
use crate::generators::docker::docker::Docker;
use chrono::{Utc};
use crate::generators::project::project::Project;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub last_updated: String,

    pub project: Project,

    pub docker: Option<Docker>,
    pub ci_cd: Option<CiCd>,
}


impl Config {
    pub fn new(docker: Option<Docker>, ci_cd: Option<CiCd>) -> Config {
        Config {
            last_updated: Utc::now().to_string(),
            project: Project {
                name: "CHF".to_string(),
                description: "CHF - marketplace for money and NFT TOKEN only EVM".to_string(),
            },
            docker,
            ci_cd,
        }
    }


    pub fn load_from_file() -> Result<Config, Box<dyn std::error::Error>> {
        let current_dir = env::current_dir()?;
        let config_path = current_dir.join("ucw.config.json");

        if !config_path.exists() {
            return Err(format!("Config file not found: {:?}", config_path).into());
        }

        let mut file = fs::File::open(&config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get the current directory
        let current_dir = env::current_dir()?;
        let config_path = current_dir.join("ucw.config.json");

        // Serialize the Config object to a JSON string
        let contents = serde_json::to_string_pretty(&self)?;

        // Write the JSON string to the file
        let mut file = fs::File::create(&config_path)?;
        file.write_all(contents.as_bytes())?;

        if let Some(docker) = &self.docker {
            docker.save_to_file()?;
        }

        Ok(())
    }

    pub fn update(&mut self, docker: Option<Docker>, ci_cd: Option<CiCd>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(docker_config) = docker {
            self.docker = Some(docker_config);
        }
        if let Some(ci_cd_config) = ci_cd {
            self.ci_cd = Some(ci_cd_config);
        }
        self.last_updated = Utc::now().to_string();
        self.save_to_file()
    }
}
