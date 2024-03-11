# A CLI BASED TODO LIST APPLICATION

This is an intuitive and easy to use CLI todo list application that allows users to add, remove, and list tasks. The application is built in Rust and stores the tasks in a .json file and allows users to interact with the tasks using the terminal. The application is built using the `serde` crates to handle command line arguments and serialize and deserialize the data to and from the .json file.

## Installation and Setup

### Run application in terminal

1. Clone the repository
2. Create a .json file in the root directory of the project to store the data
3. Remove the `file_path` using env var and replace it with the path to the .json file
4. Run `cargo run` to start the application

### Run application as a binary executable (system-wide)

1. Clone the repository
2. Run `cargo build --release` to build the binary executable
3. Add binary to `usr/local/bin` using `sudo mv /PATH/TO/FILE/todolist/target/release/todolist /usr/local/bin/`
4. Add env var to the .json file path using `export TODO_LIST_FILE_PATH=/PATH/TO/FILE/todolist.json`. Use `nano ~/.bash_profile` to add export and `source ~/.bash_profile` to apply changes
5. Run `todolist` to start the application
