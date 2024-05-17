use std::io::{BufRead, BufReader, Read};
use std::process::Command;
use std::time::Duration;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use crate::generators::javascript::package_manager::PackageManager;

pub struct Nest {
    pub framework_name: String,
    pub package_manager: PackageManager,
}

impl Nest {
    pub fn new(package_manager: PackageManager) -> Self {
        Nest {
            framework_name: "NestJS".to_string(),
            package_manager,
        }
    }

    pub fn create(&self, project_name: String) {
        let directory_prompt = dialoguer::Input::<String>::new()
            .with_prompt("Specify the directory to create the project in".cyan().bold().to_string())
            .default(project_name)
            .interact_text()
            .unwrap();



        let output = Command::new(self.package_manager.get_execution())
            .arg("@nestjs/cli")
            .arg("new")
            .arg(&directory_prompt)
            .arg("-p")
            .arg(self.package_manager.as_str())
            .output()
            .expect("Failed to create nestjs project");
    }

}

// fn create_dockerfile(directory: &str) {
//     let dockerfile_content = r#"
//     FROM node:14
//
//     # Create app directory
//     WORKDIR /usr/src/app
//
//     # Install app dependencies
//     COPY package*.json ./
//     RUN npm install
//
//     # Bundle app source
//     COPY . .
//
//     EXPOSE 3000
//     CMD [ "npm", "run", "start:prod" ]
//     "#;
//
//     let path = std::path::Path::new(directory).join("Dockerfile");
//     let mut file = std::fs::File::create(path).expect("Failed to create Dockerfile");
//     file.write_all(dockerfile_content.as_bytes()).expect("Failed to write to Dockerfile");
//
//     println!("Dockerfile created successfully in {}", directory);
// }
