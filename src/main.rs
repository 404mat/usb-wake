use std::io::{self, stdout, Stdout};

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

type Backend = CrosstermBackend<Stdout>;

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    let res = run_app(&mut terminal);
    restore_terminal(&mut terminal)?;
    if let Err(err) = res {
        println!("{err:?}");
    }
    Ok(())
}

fn setup_terminal() -> Result<Terminal<Backend>> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<Backend>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn run_app(terminal: &mut Terminal<Backend>) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(size);

    // Title
    let title_line = Line::from(Span::styled(
        "USB/Bluetooth Wakeup Manager",
        Style::default()
            .fg(Color::Black)
            .bg(Color::Blue)
            .add_modifier(ratatui::style::Modifier::BOLD),
    ));

    // Calculate center position for the title
    let title_width = "USB/Bluetooth Wakeup Manager".len() as u16;
    let center_x = (chunks[0].width.saturating_sub(title_width)) / 2;
    let center_y = chunks[0].y + chunks[0].height / 2;

    // Render the title line at the center
    f.render_widget(title_line, Rect::new(chunks[0].x + center_x, center_y, title_width, 1));

    // Main content area (placeholder for device list)
    let content = Paragraph::new("Device list will be displayed here...")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Devices"));
    f.render_widget(content, chunks[1]);

    // Help bar
    let help = Paragraph::new("Press 'q' to quit")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[2]);
}
