mod cli;
mod handlers;
mod logger;
mod observer;
mod utils;
use cli::{Cli, Command};

/// ### Handle the command
fn handle_command(command: Command) {
    match command {
        Command::Start(cmd) => handlers::handle_start(cmd),
        Command::Stop(cmd) => handlers::handle_stop(cmd),
        Command::Add(cmd) => handlers::handle_add_task(cmd),
        Command::List(cmd) => handlers::handle_list_tasks(cmd),
        Command::Remove(cmd) => handlers::handle_remove_task(cmd),
        Command::Run(cmd) => handlers::handle_run_task(cmd),
    }
}

fn main() {
    let cli = Cli::new();
    handle_command(cli.command);
}
