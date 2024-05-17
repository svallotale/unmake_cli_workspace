use std::io::BufReader;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};

#[derive(Serialize, Deserialize)]
pub struct DockerItem {
    pub name: String,
    pub image: String,
    pub volumes: Vec<String>,
    pub environment: Vec<String>,
    pub ports: Vec<String>,
    pub command: Vec<String>
}


impl DockerItem {

    pub fn new(name: String, image: String, volumes: Vec<String>, environment: Vec<String>, ports: Vec<String>, command: Vec<String>) -> Self {
        DockerItem {
            name,
            image,
            volumes,
            environment,
            ports,
            command,
        }
    }

    pub fn get_template_names() -> Vec<String> {
        let templates_path = "./templates";
        let templates = fs::read_dir(templates_path).unwrap();
        let mut template_names = vec![];
        for template in templates {
            let entry = template.unwrap();
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "json" {
                        let file = File::open(&path).unwrap();
                        let reader = BufReader::new(file);
                        let docker_item = serde_json::from_reader::<BufReader<File>, DockerItem>(reader).unwrap();
                        template_names.push(docker_item.name);
                    }
                }
            }
        }
        template_names
    }

}
