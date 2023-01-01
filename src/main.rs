use create_menu::create_menu;
use ncurses::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::prelude::*;
mod create_menu;
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    description: String,
    completed: bool,
}

const REGULAR: i16 = 0;
const SELECTED: i16 = 1;

fn main() {
    initscr();
    noecho();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    
    start_color();
    
    init_pair(REGULAR, COLOR_WHITE, COLOR_BLACK);
    init_pair(SELECTED, COLOR_BLACK, COLOR_WHITE);

    let menu: [&str; 4] = ["Menu: ", "List/Edit/Remove", "Add", "Exit"];
    let mut repeat = true;

    while repeat == true {
        clear();
        let option = create_menu::create_menu(&mut menu.to_vec());

        if option == 1 {
            list_menu();
        } else if option == 2 {
            add();
        } else if option == 3 {
            repeat = false;
        }
    }

    endwin();
}

fn add() {
    let mut name: String = String::new();
    let mut description: String = String::new();

    clear();
    echo();

    addstr("Name: ");
    refresh();
    getstr(&mut name);

    clear();

    addstr("Description: ");
    refresh();
    getstr(&mut description);

    let task = Task {
        name: name,
        description: description,
        completed: false,
    };

    let mut tasks: Vec<Task> = read_db();
    tasks.push(task);
    let json = serde_json::to_string(&tasks).unwrap();
    fs::write("src/db.json", json).expect("Unable to write file");
    noecho();
}

fn read_db() -> Vec<Task> {
    let mut file = fs::File::open("src/db.json").expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let tasks: Vec<Task> = serde_json::from_str(&contents).expect("JSON was not well-formatted");
    tasks
}

fn list_menu() -> u32 {
    initscr();
    clear();
    noecho();

    let menu: [&str; 4] = ["Options: ", "All", "Completed", "Pending"];
    let option = create_menu::create_menu(&mut menu.to_vec());

    let tasks: Vec<Task> = read_db();
    clear();
    for task in tasks {
        if option == 1 {
            addstr(&get_task(task));
            addstr("\n");
        } else if task.completed == true && option == 2 {
            addstr(&get_task(task));
            addstr("\n");
        } else if task.completed == false && option == 3 {
            addstr(&get_task(task));
            addstr("\n");
        }
    }
    getch();
    1
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
