use std::io::{self, stdout, Write};

use anyhow::Result;
use arboard::Clipboard;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Note,
    Ai,
    Clipboard,
    Bricks,
}

impl Mode {
    fn label(self) -> &'static str {
        match self {
            Self::Note => "NOTE",
            Self::Ai => "AI",
            Self::Clipboard => "CLIPBOARD",
            Self::Bricks => "BRICKS",
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Note => Self::Ai,
            Self::Ai => Self::Clipboard,
            Self::Clipboard => Self::Bricks,
            Self::Bricks => Self::Note,
        }
    }

    fn previous(self) -> Self {
        match self {
            Self::Note => Self::Bricks,
            Self::Ai => Self::Note,
            Self::Clipboard => Self::Ai,
            Self::Bricks => Self::Clipboard,
        }
    }
}

struct App {
    mode: Mode,
    note: String,
    command: String,
    status: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            mode: Mode::Note,
            note: "# Scrib\n\nYour ideas live here.\n\nSwipe or use Tab to switch the command surface.\n"
                .into(),
            command: String::new(),
            status: "Ready".into(),
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend)?;
    let result = run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::default();

    loop {
        terminal.draw(|frame| draw(frame, &app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if handle_key(&mut app, key)? {
                        return Ok(());
                    }
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }
}

fn handle_key(app: &mut App, key: KeyEvent) -> Result<bool> {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Ok(true);
    }

    match key.code {
        KeyCode::Tab => app.mode = app.mode.next(),
        KeyCode::BackTab => app.mode = app.mode.previous(),
        KeyCode::Left => app.mode = app.mode.previous(),
        KeyCode::Right => app.mode = app.mode.next(),
        KeyCode::Enter => execute_surface(app)?,
        KeyCode::Backspace => {
            app.command.pop();
        }
        KeyCode::Char(c) => app.command.push(c),
        _ => {}
    }

    Ok(false)
}

fn execute_surface(app: &mut App) -> Result<()> {
    let input = app.command.trim().to_owned();
    if input.is_empty() {
        return Ok(());
    }

    match app.mode {
        Mode::Note => {
            app.note.push_str(&format!("\n{}\n", input));
            app.status = "Added to note".into();
        }
        Mode::Ai => {
            app.status = format!("AI request queued: {input}");
        }
        Mode::Clipboard => {
            let mut clipboard = Clipboard::new()?;
            clipboard.set_text(input)?;
            app.status = "Copied to clipboard".into();
        }
        Mode::Bricks => {
            app.status = format!("Brick proposal: {input}");
        }
    }

    app.command.clear();
    Ok(())
}

fn draw(frame: &mut ratatui::Frame<'_>, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3), Constraint::Length(1)])
        .split(frame.area());

    let note = Paragraph::new(app.note.as_str())
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::Gray));
    frame.render_widget(note, layout[0]);

    let prompt = match app.mode {
        Mode::Note => "> note",
        Mode::Ai => "> ask AI to edit this note",
        Mode::Clipboard => "> find in clipboard",
        Mode::Bricks => "> describe the Linux task",
    };

    let bar = Paragraph::new(Line::from(vec![
        Span::styled(
            format!(" {} ", app.mode.label()),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::raw(prompt),
        Span::raw(": "),
        Span::styled(&app.command, Style::default().fg(Color::White)),
    ]))
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(bar, layout[1]);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" ←/→ ", Style::default().fg(Color::Cyan)),
        Span::raw("switch  "),
        Span::styled("Tab", Style::default().fg(Color::Cyan)),
        Span::raw(" mode  •  "),
        Span::raw(&app.status),
    ]));
    frame.render_widget(footer, layout[2]);

    let _ = io::stdout().flush();
}
