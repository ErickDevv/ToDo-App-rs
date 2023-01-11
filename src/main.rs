use serde::{Deserialize, Serialize};
use serde_json;

use std::fs;
use std::io::prelude::*;
use std::vec;

use rsmenuu::create_menu;
use rsmenuu::instructions_off;
use rsmenuu::Key;
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
    let mut exit_menu = false;

    while exit_menu == false {
        let options: Vec<&str> = vec!["Add", "List/Edit/Remove"];
        let keys: Vec<Key> = vec![Key {
            key: 'e',
            description: String::from("Press 'e' to exit"),
        }];

        let menu_results: MenuResult = create_menu("Menú", options, keys, true);
        if menu_results.key == 'e' {
            exit_menu = true;
        } else if menu_results.index == 0 {
            add();
        } else if menu_results.index == 1 {
            list_edit_remove();
        }
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

    let options: Vec<&str> = task_names.iter().map(|s| &**s).collect();

    let keys: Vec<Key> = vec![
        Key {
            key: 'e',
            description: String::from("Press 'e' to exit"),
        },
        Key {
            key: 'r',
            description: String::from("Press 'r' to delete"),
        },
    ];
    let menu_results: MenuResult = create_menu("Tasks", options, keys, false);

    if menu_results.key == 'e' {
        main_menu();
    } else if menu_results.key == 'r' {
        let mut tasks: Vec<Task> = read_db();
        tasks.remove(menu_results.index);
        update_db(tasks);
        list_edit_remove();
    }
}

fn add() {
    println!("Name: ");

    let mut name = String::new();

    std::io::stdin()
        .read_line(&mut name)
        .expect("Error reading the name");

    println!("Description: ");

    let mut description = String::new();

    std::io::stdin()
        .read_line(&mut description)
        .expect("Error reading the description");

    let task = Task {
        name: name.trim().to_string(),
        description: description.trim().to_string(),
        completed: false,
    };

    let mut tasks: Vec<Task> = read_db();
    tasks.push(task);

    update_db(tasks);
}

fn update_db(tasks: Vec<Task>) {
    let json = serde_json::to_string(&tasks).expect("Error serializing the task");
    fs::write("src/db.json", json).expect("Error writing the task");
}

fn read_db() -> Vec<Task> {
    let mut file = fs::File::open("src/db.json").expect("Error opening the file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Error reading the file");

    let tasks: Vec<Task> = serde_json::from_str(&contents).expect("Error deserializing the task");
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
