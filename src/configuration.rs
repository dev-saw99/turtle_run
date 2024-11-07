use crate::utils;
use serde::{Deserialize, Serialize};
/// Configuration struct
///
/// This struct is used to store the configuration of the application
#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    // api version
    version: String,
    //  workspace path
    pub workspace: String,
    // log level
    pub log_level: String,
}

impl Configuration {
    /// ### Default
    ///
    /// This function creates a default configuration
    pub fn default() -> Self {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| String::from("/home/user"));
        let default_file_path = format!("{}/.config/turtle_run/config.yaml", home_dir);
        if !std::path::Path::new(&default_file_path).exists() {
            return Configuration {
                version: String::from("v1"),
                workspace: String::from("/tmp/turtle_run/workspace"),
                log_level: String::from("info"),
            };
        }
        Configuration::new(&default_file_path)
    }

    /// ### Create a new configuration
    ///
    /// This function creates a new configuration
    pub fn new(config_file_path: &String) -> Self {
        let mut config = Configuration {
            version: String::from(""),
            workspace: String::from(""),
            log_level: String::from(""),
        };
        let mut content: String = String::from("");

        match utils::read_yaml_file(config_file_path) {
            Ok(c) => content = c,
            Err(e) => println!(
                "Error occurred while reading the configuration file: {:?}, Please configure by running `turtle_run configure` command", 
                e
            ),
        };

        // parse the content
        if !content.is_empty() {
            match serde_yaml::from_str::<Configuration>(&content) {
                Ok(parsed_config) => {
                    config = parsed_config;
                }
                Err(e) => {
                    println!("Error parsing configuration file: {:?}", e);
                }
            }
            // write the configuration to default config file
            let home_dir = std::env::var("HOME").unwrap_or_else(|_| String::from("/home/user"));
            let config_dir = format!("{}/.config/turtle_run", home_dir);
            std::fs::create_dir_all(&config_dir).unwrap_or_else(|e| {
                println!("Error creating config directory: {:?}", e);
            });
            let config_path = format!("{}/config.yaml", config_dir);
            if let Err(e) = std::fs::write(&config_path, content) {
                println!("Error writing configuration file: {:?}", e);
            }
        }

        config
    }
}
