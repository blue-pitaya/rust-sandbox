use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        //.margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(60),
                Constraint::Percentage(30),
            ]
            .as_ref(),
        )
        .split(f.size());
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default().title("Block 2").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
    let block = Block::default().title("Block 3").borders(Borders::ALL);
    f.render_widget(block, chunks[2]);
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        ui(f);
        //let size = f.size();
        //let block = Block::default().title("Block").borders(Borders::ALL);

        //f.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
