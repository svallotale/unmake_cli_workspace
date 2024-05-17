use colored::Colorize;
use dialoguer::Select;

pub enum PackageManager {
    Bun,
    Npm,
    Yarn,
    Pnpm,
}

impl PackageManager {
    pub fn as_str(&self) -> &'static str {
        match self {
            PackageManager::Bun => "bun",
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "yarn",
            PackageManager::Pnpm => "pnpm",
        }
    }
    
    pub fn get_execution(&self) -> &'static str {
        match self {
            PackageManager::Bun => "bunx",
            PackageManager::Npm => "npx",
            PackageManager::Yarn => "yarn",
            PackageManager::Pnpm => "pnpm",
        }
    }
    
    pub fn how_package_manager() -> Self {
        let items = vec![
            "bun",
            "npm",
            "yarn",
            "pnpm",
        ];
        let selection = Select::new()
            .with_prompt("Package manager".bold().cyan().to_string())
            .items(&items)
            .default(0)
            .interact()
            .unwrap();

        match items[selection] { 
            "bun" => PackageManager::Bun,
            "npm" => PackageManager::Npm,
            "yarn" => PackageManager::Yarn,
            "pnpm" => PackageManager::Pnpm,
            _ => PackageManager::Bun,    
        }
    }

}
