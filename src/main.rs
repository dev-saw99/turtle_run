/*
 * SPDX-License-Identifier: Apache License 2.0
 * More licensing information can be found in the project LICENSE file
 * Author: Sonu Kumar Saw
 * Email: sonukumarsaw66@gmail.com
 */

mod cli;
mod configuration;
mod handlers;
mod logger;
mod observer;
mod utils;
use cli::{Cli, Command};
use handlers::Handlers;

/// ### Handle the command
fn handle_command(command: Command) {
    let mut command_handler = Handlers::new();

    match command {
        Command::Configure(cmd) => command_handler.handle_configure(cmd),
        Command::Start(cmd) => command_handler.handle_start(cmd),
        Command::Stop(cmd) => command_handler.handle_stop(cmd),
        Command::Add(cmd) => command_handler.handle_add_task(cmd),
        Command::List(cmd) => command_handler.handle_list_tasks(cmd),
        Command::Remove(cmd) => command_handler.handle_remove_task(cmd),
        Command::Run(cmd) => command_handler.handle_run_task(cmd),
    }
}

fn main() {
    let cli = Cli::new();
    handle_command(cli.command);
}
