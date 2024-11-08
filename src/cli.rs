/*
 * SPDX-License-Identifier: Apache License 2.0
 * More licensing information can be found in the project LICENSE file
 * Author: Sonu Kumar Saw
 * Email: sonukumarsaw66@gmail.com
 *
 * Copyright (c) 2024
 * All rights reserved.
 */

use clap::{Parser, Subcommand};

/// ### Represents the main command line interface for turtle_run.
///
/// run `turtle_run --help` to see the help message.:
/// ```shell
/// turtle_run --help
///
#[derive(Parser, Debug)]
#[clap(
    name = "turtle_run",
    about = "A command line based task scheduler, that enables users to add, execute, and manage times bases tasks.",
    version = "0.0.1",
    author = "Sonu Kumar Saw <dev-saw99>"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

/// ### Represents subcommands offered by turtle_run cli.
/// #### commands
/// * `Start` - Start the task scheduler.
/// * `Stop` - Stop the task scheduler.
/// * `Add` - Add a new task.
/// * `List` - List all tasks.
/// * `Remove` - Remove a task.
/// * `Run` - Run a task.
#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(about = "Start the task scheduler")]
    Start(StartCommand),
    #[clap(about = "Stop the task scheduler")]
    Stop(StopCommand),
    #[clap(about = "Add a new task")]
    Add(AddTaskCommand),
    #[clap(about = "List all tasks")]
    List(ListCommand),
    #[clap(about = "Remove a task")]
    Remove(RemoveCommand),
    #[clap(about = "Run a task")]
    Run(RunCommand),
    #[clap(about = "Configure the task scheduler")]
    Configure(ConfigureCommand),
}

/// ### Represents a command to configure the task scheduler.
#[derive(Parser, Debug)]
pub struct ConfigureCommand {
    #[clap(short, long, help = "The path to the config file")]
    pub config: String,
}

/// ### Represents a command to start the task scheduler.
#[derive(Parser, Debug)]
pub struct StartCommand {}

/// ### Represents a command to stop the task scheduler.
#[derive(Parser, Debug)]
pub struct StopCommand {}

/// ### Represents a command to add a task with specific parameters.
///
/// #### Fields
///
/// * `command` - The command to run.
/// * `time` - The time when to run the command.
/// * `repeat` - Optional parameter to specify if the task should be repeated.
#[derive(Parser, Debug)]
pub struct AddTaskCommand {
    #[clap(short, long, help = "The command to run")]
    pub command: String,
    #[clap(short, long, help = "The time, when to run the command")]
    pub time: String,
    #[clap(short, long, help = "Repeat the task")]
    pub repeat: Option<String>,
}

/// ### Represents a command to list all tasks.
///
#[derive(Parser, Debug)]
pub struct ListCommand {}

/// ### Represents a command to remove a task with specific id.
///
/// #### Fields
///
/// * `id` - Task Id which needs to be removed.
#[derive(Parser, Debug)]
pub struct RemoveCommand {
    #[clap(short, long, help = "The ID of the task to remove")]
    pub id: u32,
}

/// ### Represents a command to run a task with specific id.
///
/// #### Fields
///
/// * `id` - Task Id which needs to be removed.
#[derive(Parser, Debug)]
pub struct RunCommand {
    #[clap(short, long, help = "The ID of the task to run")]
    pub id: u32,
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }
}
