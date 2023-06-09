use ncurses::*;

fn main() {
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();

    // Get the size of the terminal window
    let mut rows = 0;
    let mut cols = 0;
    getmaxyx(stdscr(), &mut rows, &mut cols);

    // Calculate the position for the center of the screen
    let x = cols / 2;
    let y = rows / 2;

    // Print "Hello, World!" at the center of the screen
    mvprintw(y, x - 6, "Hello, World!");

    refresh();
    getch();
    endwin();
}
