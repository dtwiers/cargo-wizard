use std::{error::Error, time::Duration};

use crates_io_api::Crate;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io::Stdout;
mod api;
mod home_screen;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    let crates = api::search_packages("rust").await?;
    run(&mut terminal, &crates)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}
fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = std::io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, crates: &Vec<Crate>) -> Result<(), Box<dyn Error>> {
    Ok(loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello World!");
            let my_list: Vec<ListItem> = crates.iter().map(|c| ListItem::new(c.name.clone())).collect();
            let list = List::new(my_list)
                .block(Block::default().title("List").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");
            frame.render_widget(greeting, frame.size());
            frame.render_widget(list, frame.size());
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    })
}
