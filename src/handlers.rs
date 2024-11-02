use crate::cli::{
    AddTaskCommand, ListCommand, RemoveCommand, RunCommand, StartCommand, StopCommand,
};
use crate::logger;
use crate::observer;

/// ### Handle the StartCommand
pub fn handle_start(_command: StartCommand) {
    logger::init_file_logger();
    log::info!("Starting the task scheduler");

    // spawn the observer thread
    observer::spawn_observer();
}

/// ### Handle the StopCommand
pub fn handle_stop(_command: StopCommand) {
    logger::init_file_logger();
    log::info!("Starting the task scheduler");

    // Implement the logic to stop the task scheduler
}

/// ### Handle the AddTaskCommand
pub fn handle_add_task(command: AddTaskCommand) {
    // Implement the logic to add a new task
    logger::init_file_and_console_logger();
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
pub fn handle_list_tasks(_command: ListCommand) {
    // Implement the logic to list all tasks
    println!("Listing all tasks");
}

/// ### Handle the RemoveCommand
pub fn handle_remove_task(command: RemoveCommand) {
    // Implement the logic to remove a task
    println!("Removing task with ID: {}", command.id);
}

/// ### Handle the RunCommand
pub fn handle_run_task(command: RunCommand) {
    // Implement the logic to run a task
    println!("Running task with ID: {}", command.id);
}
