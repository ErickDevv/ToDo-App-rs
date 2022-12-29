use ncurses::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::prelude::*;

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

    let menu: [&str; 6] = ["Menu: ", "List", "Add", "Edit", "Remove", "Exit"];
    let mut repeat = true;

    while repeat == true {
        clear();
        let option = crate_menu(&mut menu.to_vec());

        if option == 1 {
            list_menu();
        } else if option == 5 {
            repeat = false;
        }
    }

    endwin();
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
    let option = crate_menu(&mut menu.to_vec());

    if option == 1 {
        clear();
        let tasks: Vec<Task> = read_db();
        for task in tasks {
            addstr(&task.name);
            addstr("\n");
        }
        refresh();
        getch();
    }
    1
}

fn crate_menu(options: &mut [&str]) -> i16 {
    mv(options.len() as i32, 1);
    let mut current_option: i16 = 1;
    let mut repeat = false;
    let mut final_option: i16 = 0;
    while repeat == false {
        for (i, todo) in options.iter().enumerate() {
            let pair = if i == current_option as usize {
                SELECTED
            } else {
                REGULAR
            };
            attron(COLOR_PAIR(pair));
            mv(i as i32, 1);
            addstr(*todo);
            attroff(COLOR_PAIR(pair));
        }

        refresh();

        let key = getch();
        if key == 101 {
            repeat = true;
        } else if key == 119 && current_option > 1 {
            current_option -= 1;
        } else if key == 115 && current_option < options.len() as i16 - 1 {
            current_option += 1;
        } else if key == 100 {
            final_option = current_option;
            repeat = true;
        }
    }

    final_option
}
