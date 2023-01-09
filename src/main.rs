use serde::{Deserialize, Serialize};
use serde_json;

use std::fs;
use std::io::prelude::*;

use rsmenuu::create_menu;
use rsmenuu::instructions_off;
use rsmenuu::MenuResult;
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    description: String,
    completed: bool,
}

fn main() {
    main_menu();
}

fn main_menu() {
    let options: Vec<&str> = vec!["Add", "List/Edit/Remove"];

    let menu_results: MenuResult = create_menu("Menú", options, instructions_off(), true);

    if menu_results.index == 0 {
        //add();
    } else if menu_results.index == 1 {
        list_edit_remove();
    }
}

fn list_edit_remove() {
    let options: Vec<&str> = vec!["All", "Completed", "Uncompleted"];
    let menu_results: MenuResult =
        create_menu("List/Edit/Remove", options, instructions_off(), true);
    let tasks: Vec<Task> = read_db();
    let mut task_names: Vec<String> = Vec::new();
    for task in tasks {
        if menu_results.index == 0 {
            task_names.push(get_task(task));
        } else if menu_results.index == 1 {
            if task.completed == true {
                task_names.push(get_task(task));
            }
        } else if menu_results.index == 2 {
            if task.completed == false {
                task_names.push(get_task(task));
            }
        }
    }

    for task in task_names {
        println!("{}", task);
    }
}

fn read_db() -> Vec<Task> {
    let mut file = fs::File::open("src/db.json").expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let tasks: Vec<Task> = serde_json::from_str(&contents).expect("JSON was not well-formatted");
    tasks
}

fn get_task(task: Task) -> String {
    let pre: String = if task.completed == true {
        "[x] ".to_owned()
    } else {
        "[ ] ".to_owned()
    };
    let name: String = task.name.to_owned();
    let owned_name: String = format!("{}{}", pre, name);
    owned_name
}
