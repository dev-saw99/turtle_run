/*
 * SPDX-License-Identifier: Apache License 2.0
 * More licensing information can be found in the project LICENSE file
 * Author: Sonu Kumar Saw
 * Email: sonukumarsaw66@gmail.com
 */

use std::fs;

// get the yaml file path
pub fn get_task_yaml_file_path() -> String {
    "/tmp/turtle_run/turtle_run.task.yaml".to_string()
}

pub fn create_task_yaml_file(file_path: String) {
    let file_path = file_path.as_str();
    match fs::write(file_path, "") {
        Ok(_) => log::info!("Task file created: {}", file_path),
        Err(e) => log::error!("Error creating task file: {}, error: {}", file_path, e),
    }
}

// function to read yaml file and return the content
pub fn read_yaml_file(file_path: String) -> Result<String, String> {
    let file_path = file_path.as_str();
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Error reading file: {}, error: {}", file_path, e)),
    }
}
