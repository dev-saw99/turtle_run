/*
 * SPDX-License-Identifier: Apache License 2.0
 * More licensing information can be found in the project LICENSE file
 * Author: Sonu Kumar Saw
 * Email: sonukumarsaw66@gmail.com
 *
 * Copyright (c) 2024
 * All rights reserved.
 */

use crate::cli::{
    AddTaskCommand, ConfigureCommand, ListCommand, RemoveCommand, RunCommand, StartCommand,
    StopCommand,
};

use crate::configuration::Configuration;
use crate::logger;
use crate::observer::Observer;

#[derive(Debug)]
pub struct Handlers {
    observer: Observer,
    pub configuration: Configuration,
}

impl Handlers {
    pub fn new() -> Handlers {
        let configuration = Configuration::default();
        let handler = Handlers {
            observer: Observer::new(configuration.clone()),
            configuration: configuration,
        };

        handler
    }

    /// ### Handle the StartCommand
    pub fn handle_start(&self, _command: StartCommand) {
        logger::init_file_logger(&self.configuration.workspace, &self.configuration.log_level);
        log::info!("Starting the task scheduler");
        log::debug!("Starting the observer thread");

        // spawn the observer thread
        Observer::spawn_observer(self.observer.clone().into());
    }

    /// ### Handle the StopCommand
    pub fn handle_stop(&self, _command: StopCommand) {
        logger::init_file_logger(&self.configuration.workspace, &self.configuration.log_level);
        log::info!("Starting the task scheduler");

        // Implement the logic to stop the task scheduler
    }

    /// ### Handle the AddTaskCommand
    pub fn handle_add_task(&self, command: AddTaskCommand) {
        // Implement the logic to add a new task
        logger::init_file_and_console_logger(
            &self.configuration.workspace,
            &self.configuration.log_level,
        );
        log::info!(
            "Adding task with command: {}, time: {}, repeat: {:?}",
            command.command,
            command.time,
            command.repeat
        );
        log::error!("Error adding task");
        log::warn!("Warning adding task");
    }

    /// ### Handle the ListCommand
    pub fn handle_list_tasks(&self, _command: ListCommand) {
        // Implement the logic to list all tasks
        log::info!("Listing all tasks");
    }

    /// ### Handle the RemoveCommand
    pub fn handle_remove_task(&self, command: RemoveCommand) {
        // Implement the logic to remove a task
        log::info!("Removing task with ID: {}", command.id);
    }

    /// ### Handle the RunCommand
    pub fn handle_run_task(&self, command: RunCommand) {
        // Implement the logic to run a task
        log::info!("Running task with ID: {}", command.id);
    }

    pub fn handle_configure(&mut self, command: ConfigureCommand) {
        // Implement the logic to configure the task scheduler
        println!("Configuring task scheduler : {:?}", &command);
        self.configuration = Configuration::new(&command.config);
    }
}
