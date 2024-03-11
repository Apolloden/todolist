use colored::*;

pub fn remove_first_word(input: &str) -> &str {
    if let Some(index) = input.find(char::is_whitespace) {
        &input[index + 1..]
    } else {
        ""
    }
}

pub fn display_options() {
    println!("{}", "-------------------".bright_yellow());
    println!(
        "{}",
        "TODO LIST HELP MENU".bright_yellow().bold().underline()
    );
    println!("  Enter the command followed by the required parameters to execute.\n");

    println!("{}", "MISCELLANEOUS COMMANDS".bright_green().bold());
    println!("  {} - display this help menu", "help".bright_blue());
    println!("  {} - exit the todo list application", "q".bright_blue());

    println!("\n{}", "TODO LIST COMMANDS".bright_green().bold());
    println!(
        "  {} - display all tasks in the todo list",
        "ls".bright_blue()
    );
    println!(
        "  {} <task name>, <task area>, <priority> - add a new task to the todo list",
        "add".bright_blue()
    );
    println!(
        "  {} <task name> - remove a task from the todo list by its name",
        "rm".bright_blue()
    );
    println!(
        "  {} <old task name> <new task name> - rename a task in the todo list",
        "rename".bright_blue()
    );
    println!(
        "  {} <new todo list name> - rename the todo list",
        "renametodo".bright_blue()
    );
    println!(
        "  {} <task field> - add a new field to the todo list",
        "newfield".bright_blue()
    );
    println!(
        "  {} <task priority> - update the priority of a task in the todo list",
        "reprio".bright_blue()
    );
    println!(
        "  {} <task name> - toggle task as active or inactive",
        "toggle".bright_blue()
    );

    println!(
        "\n{}",
        "NOTE: parameters should be separated by commas ','"
            .bright_red()
            .bold()
    );

    println!("{}", "-------------------".bright_yellow());
}
