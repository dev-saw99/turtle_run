![Turtle-Run Logo](logo.png)

Turtle-Run is a command-line task scheduler application that allows you to manage and execute scheduled tasks efficiently. The application is designed to provide an easy way to start, stop, add, list, and remove tasks.

## Features

- **Task Management**: Start, stop, add, list, and remove scheduled tasks.
- **User-Friendly CLI**: Intuitive command-line interface for task scheduling.

## Installation

To install Turtle-Run, you need to have Rust installed on your system. You can install Rust from [here](https://www.rust-lang.org/tools/install).

Clone the repository and build the project:

```sh
git clone https://github.com/yourusername/turtle_run.git
cd turtle_run
cargo build --release
```

## Usage

To use Turtle-Run, run the following command:

```sh
./target/release/turtle_run <command>
```

Replace `<command>` with one of the available commands: `start`, `stop`, `add`, `list`, `remove`, `run`.

### Task Management

The following commands are available for managing tasks:

- **`start`**: Starts the task scheduler to run in the background.
- **`stop`**: Stops the task scheduler.
- **`add`**: Adds a new task to the scheduler.
- **`list`**: Lists all scheduled tasks.
- **`remove`**: Removes a specific task from the scheduler.

#### Example Usage

To start the task scheduler, use:

```sh
./target/release/turtle_run start
```

To add a task:

```sh
./target/release/turtle_run add --command "ls" --time "12:00"
```

To list all scheduled tasks:

```sh
./target/release/turtle_run list
```

## License

This project is licensed under the Apache License 2.0. More licensing information can be found in the project LICENSE file.

## Author

Sonu Kumar Saw  
Email: [sonukumarsaw66@gmail.com](mailto:sonukumarsaw66@gmail.com)

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
