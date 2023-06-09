use ncurses::*;
use std::collections::LinkedList;

fn change_dir(key: i32, move_vec: &mut (i32, i32)) {
    let x = match key {
        KEY_UP => (0, -1),
        KEY_DOWN => (0, 1),
        KEY_LEFT => (-1, 0),
        KEY_RIGHT => (1, 0),
        _ => *move_vec,
    };

    if x.0 == -move_vec.0 || x.1 == -move_vec.1 {
        return;
    }

    *move_vec = x;
}

fn main() {
    initscr();
    raw();
    keypad(stdscr(), true);
    nodelay(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut move_vector = (0, 1);
    let mut snake = LinkedList::new();
    snake.push_back((10, 10));

    loop {
        let key = getch();
        if key == 'q' as i32 {
            break;
        }

        change_dir(key, &mut move_vector);
        let head = snake.front().expect("Err");
        let new_head = (head.0 + move_vector.0, head.1 + move_vector.1);

        snake.push_front(new_head);
        snake.pop_back();

        //draw
        clear();
        for &(x, y) in &snake {
            mvprintw(y, x, "*");
        }
        refresh();

        napms(50);
    }

    endwin();
}
