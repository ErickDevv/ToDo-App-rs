use ncurses::*;

const REGULAR: i16 = 0;
const SELECTED: i16 = 1;

pub fn create_menu(options: &mut [&str]) -> i16 {
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
