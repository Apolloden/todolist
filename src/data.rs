use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    name: String,
    field: String,
    prio: i32,
    active: bool,
    id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    name: String,
    tasks: Vec<Task>,
}

impl TodoList {
    pub fn new(todo_list_name: &str) -> TodoList {
        TodoList {
            name: todo_list_name.to_string(),
            tasks: Vec::new(),
        }
    }

    pub fn print_tasks(&self) {
        println!("{}", "📋 Todo List".bright_yellow().bold().underline());
        println!("{}", self.name.bright_green().bold().underline());
        println!("{}", "---------------------------".bright_yellow());
        for task in self.tasks.iter() {
            println!("    {}", format!("🔹 {}", task.name).bright_blue().bold());
            println!(
                "    {}",
                format!("📍 AREA: {}", task.field).bright_magenta()
            );
            println!(
                "    {}",
                format!("⭐️ PRIORITY: {}", task.prio).bright_cyan()
            );
            let active_status = if task.active {
                "✅ ACTIVE".green()
            } else {
                "❌ INACTIVE".red()
            };
            println!("    {}", active_status.bold());
            println!("{}", "---------------------------".bright_yellow());
        }
    }

    pub fn find_task_by_name(&self, target_name: &str) -> Option<&Task> {
        self.tasks.iter().find(|task| task.name == target_name)
    }

    pub fn find_task_index_by_name(&self, target_name: &str) -> Option<usize> {
        self.tasks.iter().position(|task| task.name == target_name)
    }

    pub fn add(&mut self, name: String, field: String, prio: i32, active: bool) {
        let new_task = Task {
            name: name,
            field: field,
            prio: prio,
            active: active,
            id: get_new_task_id(self),
        };
        self.tasks.push(new_task);
    }

    pub fn remove_by_name(&mut self, target_name: &str) {
        if let Some(index) = self.tasks.iter().position(|task| task.name == target_name) {
            self.tasks.remove(index);
        }
    }

    pub fn rename_task(&mut self, target_name: &str, new_name: String) {
        if let Some(index) = self.find_task_index_by_name(target_name) {
            self.tasks[index].name = new_name;
        } else {
            println!("Error: Task '{}' not found", target_name);
        }
    }

    pub fn rename_todo_list(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn new_field(&mut self, target_name: &str, new_field: String) {
        if let Some(index) = self.find_task_index_by_name(target_name) {
            self.tasks[index].field = new_field;
        } else {
            println!("Error: Task '{}' not found", target_name);
        }
    }

    pub fn reprio(&mut self, target_name: &str, new_prio: i32) {
        if let Some(index) = self.find_task_index_by_name(target_name) {
            self.tasks[index].prio = new_prio;
        } else {
            println!("Error: Task '{}' not found", target_name);
        }
    }

    pub fn toggle_active(&mut self, target_name: &str) {
        if let Some(index) = self.find_task_index_by_name(target_name) {
            self.tasks[index].active = !self.tasks[index].active;
        } else {
            println!("Error: Task '{}' not found", target_name);
        }
    }
}

fn get_new_task_id(todo_list: &TodoList) -> i32 {
    match todo_list.tasks.iter().map(|task| task.id).max() {
        Some(max_id) => max_id + 1,
        None => 0,
    }
}

pub fn make_todo_list(todo_list_name: &str) -> TodoList {
    TodoList::new(todo_list_name)
}
