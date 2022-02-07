use crate::game::GameState;

mod game;

fn main() {
    let mut game = GameState::new(100, 50);

    ncurses::initscr();

    loop {
        game.step();
        ncurses::mvprintw(0, 0, game.render().as_str());
        ncurses::refresh();
        match ncurses::getch() {
            ncurses::KEY_CANCEL => break,
            _ => { }
        }
    }

    ncurses::endwin();
}
