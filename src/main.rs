use std::env;

// JSON
use serde_json::json;
use std::fs::File;
use std::io::{self, Read, Write};

mod helper;
use helper::display_options;
mod data;
use data::make_todo_list;
use data::TodoList;

fn select_option(input: &str, todo_list: &mut TodoList) -> bool {
    // list of valid commands
    let keywords = [
        "help",
        "q",
        "ls",
        "add",
        "rm",
        "rename",
        "renametodo",
        "newfield",
        "reprio",
        "toggle",
    ];

    let first = input.split_whitespace().next().unwrap_or("");

    if !keywords.contains(&first) {
        println!("Error: First word is not a valid keyword");
        return true;
    }

    // remove keyword from input
    let input = helper::remove_first_word(input);

    // split string into words on ','
    let words: Vec<&str> = input.split(',').map(|s| s.trim()).collect();

    // Check if the format is valid
    if words.is_empty() {
        println!("Error: Input is empty");
        return true;
    }

    match first {
        "help" => {
            display_options();
            return true;
        }
        "q" => {
            return false;
        }
        "ls" => {
            todo_list.print_tasks();
            return true;
        }
        "add" => {
            if words.len() != 3 {
                println!("Error: Insufficient arguments for 'add' command");
                return true;
            }

            // Parse the priority (words[3]) to i32
            let prio: i32 = match words[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Error: Priority '{}' is not a valid integer", words[2]);
                    return true;
                }
            };

            todo_list.add(words[0].to_string(), words[1].to_string(), prio, true);
            return true;
        }
        "rm" => {
            if words.len() != 1 {
                println!("Error: Insufficient arguments for 'remove' command");
                return true;
            }

            // Find the index of the task by name
            let task_name = words[0].to_string();
            if todo_list.find_task_by_name(&task_name).is_some() {
                todo_list.remove_by_name(&task_name);
            } else {
                println!("Error: Task '{}' not found", words[0]);
            }
            return true;
        }
        "rename" => {
            if words.len() != 2 {
                println!("Error: Insufficient arguments for 'rename' command");
                return true;
            }

            todo_list.rename_task(words[0], words[1].to_string());
            return true;
        }
        "renametodo" => {
            if words.len() != 1 {
                println!("Error: Insufficient arguments for 'renametodo' command");
                return true;
            }

            todo_list.rename_todo_list(words[0].to_string());
            return true;
        }
        "newfield" => {
            if words.len() != 2 {
                println!("Error: Insufficient arguments for 'newfield' command");
                return true;
            }
            todo_list.new_field(words[0], words[1].to_string());
            return true;
        }
        "reprio" => {
            if words.len() != 2 {
                println!("Error: Insufficient arguments for 'reprio' command");
                return true;
            }

            let prio: i32 = match words[1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Error: Priority '{}' is not a valid integer", words[1]);
                    return true;
                }
            };

            todo_list.reprio(words[0], prio);
            return true;
        }
        "toggle" => {
            if words.len() != 1 {
                println!("Error: Insufficient arguments for 'newactivity' command");
                return true;
            }
            todo_list.toggle_active(words[0]);
            return true;
        }
        _ => unreachable!(), // will never happen
    }
}

fn main() {

    // get file path from environment variable
    let file_path = match env::var("TODO_LIST_FILE_PATH") { // /PATH/TO/FILE/HERE/todolist.json/
        Ok(path) => path,
        Err(_) => {
            eprintln!("Error: TODO_LIST_FILE_PATH environment variable is not set");
            return;
        }
    };

    // read from file
    let mut file = File::open(&file_path)
        .unwrap_or_else(|_| File::create(&file_path).expect("Failed to create new file"));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let mut todolist: TodoList = match serde_json::from_str(&contents) {
        Ok(list) => list,
        Err(_) => make_todo_list("My first todo list"),
    };

    // main loop
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                if !select_option(&input.trim(), &mut todolist) {
                    break;
                }
            }
            Err(error) => println!("error: {error}"),
        }
    }

    // write back to file
    let todo_list_json = json!(todolist);
    let mut file = File::create(&file_path).expect("Failed to create file");
    file.write_all(todo_list_json.to_string().as_bytes())
        .expect("Failed to write to file");
}
