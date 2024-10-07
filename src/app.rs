use std::fmt::Display;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, List, ListItem, ListState,
    },
    DefaultTerminal, Frame,
};

use crate::error::AppError;

#[derive(Debug, Default)]
enum Mode {
    #[default]
    Countries,
    Cities,
    Connection,
}

#[derive(Debug, Default)]
enum InputMode {
    #[default]
    Normal,
    Search,
}

impl Display for InputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputMode::Normal => write!(f, "Normal"),
            InputMode::Search => write!(f, "Search"),
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    countries: Vec<String>,
    cities: Vec<String>,

    connection_output: String,
    connected: bool,

    search_string: String,

    mode: Mode,
    input_mode: InputMode,
    state: ListState,

    index: usize,

    exit: bool,
}

impl App {
    pub fn init() -> Result<Self, AppError> {
        let output = std::process::Command::new("nordvpn")
            .arg("countries")
            .output()?;

        let countries: Vec<String> = String::from_utf8(output.stdout)?
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let output = std::process::Command::new("nordvpn")
            .arg("status")
            .output()?;

        let connection_status = String::from_utf8(output.stdout)?.contains("Connected");

        let mut state = ListState::default();
        state.select(Some(0));

        Ok(Self {
            countries,
            cities: Vec::new(),
            connection_output: String::new(),
            connected: connection_status,
            search_string: String::default(),
            index: 0,
            input_mode: InputMode::default(),
            mode: Mode::default(),
            state,
            exit: false,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn set_cities(&mut self) -> Result<(), AppError> {
        let output = std::process::Command::new("nordvpn")
            .arg("cities")
            .arg(&self.countries[self.index])
            .output()?;

        self.cities = String::from_utf8(output.stdout)?
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        Ok(())
    }

    fn connect(&mut self) -> Result<Mode, AppError> {
        let output = std::process::Command::new("nordvpn")
            .arg("connect")
            .arg(&self.cities[self.index])
            .output()?;

        self.connected = output.status.success();
        let command_output: Vec<String> = String::from_utf8(output.stdout)?.lines().map(|s| s.to_string()).collect();
        self.connection_output = command_output.join("\n");

        if output.status.success() {
            Ok(Mode::Connection)
        } else {
            Err(AppError::Command(output.status))
        }
    }

    fn draw(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(f.area());

        let title_text = if self.connected {
            format!(
                " nordvpn-tui - {} ",
                "Connected".to_string().bold().fg(Color::Green)
            )
        } else {
            format!(
                " nordvpn-tui - {} ",
                "Disconnected".to_string().bold().fg(Color::Red)
            )
        };

        let title = Title::from(title_text);

        let instructions = match self.input_mode {
            InputMode::Normal => Title::from(Line::from(vec![
                " Normal | ".bold(),
                " Select ".bold(),
                "<Enter>".into(),
                " Down ".bold(),
                "<J | Up>".into(),
                " Up ".bold(),
                "<K | Down>".into(),
                " Quit ".bold(),
                "<Q | Esc>".into(),
            ])),
            InputMode::Search => {
                let search_text = format!(" Search: {} | ", self.search_string);
                let instructions = vec![
                    search_text.into(),
                    " Type ".bold(),
                    "<Esc>".into(),
                    " to exit search mode ".bold(),
                    " Type ".bold(),
                    "<Backspace>".into(),
                    " to delete ".bold(),
                ];
                Title::from(Line::from(instructions).style(Style::default().fg(Color::LightYellow)))
            }
        };

        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        match self.mode {
            Mode::Countries | Mode::Cities => self.draw_lists(f, chunks[1], block),
            Mode::Connection => self.draw_connection(f, chunks[1], block),
        }
    }

    fn draw_lists(&mut self, f: &mut Frame, area: Rect, block: Block) {
        let mut list = Vec::<ListItem>::new();

        let l = match self.mode {
            Mode::Countries => self
                .countries
                .clone()
                .into_iter()
                .filter(|c| c.to_lowercase().contains(&self.search_string))
                .collect(),
            Mode::Cities => self
                .cities
                .clone()
                .into_iter()
                .filter(|c| c.to_lowercase().contains(&self.search_string))
                .collect(),
            _ => Vec::new(),
        };
        for (i, country) in l.iter().enumerate() {
            let style = if i == self.index {
                Style::default().fg(Color::LightYellow)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            list.push(ListItem::new(
                Line::from(Span::from(country.to_string()))
                    .alignment(Alignment::Center)
                    .style(style),
            ));
        }

        let list = List::new(list).block(block).highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC),
        );
        f.render_stateful_widget(list, f.area(), &mut self.state);
    }

    fn draw_connection(&mut self, f: &mut Frame, area: Rect, block: Block) {
        let text = Line::from(Span::from(self.connection_output.to_string()))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightYellow));

        let text = List::new(vec![ListItem::new(text)])
            .block(block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::ITALIC),
            );

        f.render_widget(text, f.area());
    }

    fn handle_events(&mut self) -> Result<(), AppError> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                self.handle_key_event(key_event)?
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> Result<(), AppError> {
        match self.input_mode {
            InputMode::Normal => self.handle_normal_mode(event)?,
            InputMode::Search => self.handle_search_mode(event)?,
        }
        Ok(())
    }

    fn handle_normal_mode(&mut self, event: KeyEvent) -> Result<(), AppError> {
        match event.code {
            event::KeyCode::Esc | event::KeyCode::Char('q') => self.exit = true,
            event::KeyCode::Enter => {
                self.mode = match self.mode {
                    Mode::Countries => {
                        self.state.select(Some(0));
                        self.set_cities()?;
                        self.index = 0;
                        Mode::Cities
                    }
                    Mode::Cities => self.connect()?,
                    Mode::Connection => {
                        self.index = 0;
                        Mode::Countries
                    }
                };
            }
            event::KeyCode::Down | event::KeyCode::Char('j') => self.increment_counter(),
            event::KeyCode::Up | event::KeyCode::Char('k') => self.decrement_counter(),
            event::KeyCode::Char('/') | event::KeyCode::Char('i') => {
                self.input_mode = InputMode::Search;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_search_mode(&mut self, event: KeyEvent) -> Result<(), AppError> {
        match event.code {
            event::KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
            }
            event::KeyCode::Char(c) => {
                self.search_string.push(c);
                self.index = 0;
            }
            event::KeyCode::Backspace => {
                self.search_string.pop();
                self.index = 0;
            }
            _ => {}
        }
        Ok(())
    }

    fn decrement_counter(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
        self.state.select(Some(self.index));
    }

    fn increment_counter(&mut self) {
        if self.index < self.countries.len() - 1 {
            self.index += 1;
        }
        self.state.select(Some(self.index));
    }
}
