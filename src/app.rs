use std::{fmt::Display, str::FromStr};

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

use crate::{config, error::AppError};

#[derive(Debug, Default)]
enum View {
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

    view_mode: View,
    input_mode: InputMode,
    state: ListState,

    awaiting_second_g: bool,

    country_index: usize,
    city_index: usize,

    config: config::Config,

    exit: bool,
}

impl App {
    pub fn init(config: Option<String>) -> Result<Self, AppError> {
        let config = config::Config::load(config.as_deref())?;

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
            country_index: 0,
            city_index: 0,
            input_mode: InputMode::default(),
            view_mode: View::default(),
            awaiting_second_g: false,
            state,
            config,
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

    fn set_countries(&mut self) -> Result<(), AppError> {
        let output = std::process::Command::new("nordvpn")
            .arg("countries")
            .output()?;

        self.countries = String::from_utf8(output.stdout)?
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        Ok(())
    }

    fn set_cities(&mut self) -> Result<(), AppError> {
        let output = std::process::Command::new("nordvpn")
            .arg("cities")
            .arg(&self.countries[self.country_index])
            .output()?;

        self.cities = String::from_utf8(output.stdout)?
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        Ok(())
    }

    fn connect(&mut self) -> Result<View, AppError> {
        let output = std::process::Command::new("nordvpn")
            .arg("connect")
            .arg(&self.cities[self.city_index])
            .output()?;

        self.connected = output.status.success();
        let command_output: Vec<String> = String::from_utf8(output.stdout)?
            .lines()
            .map(|s| s.to_string())
            .collect();
        self.connection_output = command_output.join("\n");

        if output.status.success() {
            Ok(View::Connection)
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
                "Connected"
                    .to_string()
                    .bold()
                    .fg(self.config.colors.connected)
            )
        } else {
            format!(
                " nordvpn-tui - {} ",
                "Disconnected"
                    .to_string()
                    .bold()
                    .fg(self.config.colors.disconnected)
            )
        };

        let title = Title::from(title_text);

        let instructions = match self.input_mode {
            InputMode::Normal => Title::from(
                Line::from(vec![
                    " Normal | ".bold(),
                    " Select ".bold(),
                    "<Enter>".into(),
                    " Down ".bold(),
                    "<J | Up>".into(),
                    " Up ".bold(),
                    "<K | Down>".into(),
                    " Quit ".bold(),
                    "<Q | Esc>".into(),
                ])
                .style(Style::default().fg(self.config.colors.normal_mode)),
            ),
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
                Title::from(
                    Line::from(instructions)
                        .style(Style::default().fg(self.config.colors.search_mode)),
                )
            }
        };

        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .bg(self.config.colors.background)
            .border_set(border::THICK);

        match self.view_mode {
            View::Countries | View::Cities => self.draw_lists(f, chunks[1], block),
            View::Connection => self.draw_connection(f, chunks[1], block),
        }
    }

    fn draw_lists(&mut self, f: &mut Frame, _area: Rect, block: Block) {
        let mut list = Vec::<ListItem>::new();

        let l = match self.view_mode {
            View::Countries => self
                .countries
                .clone()
                .into_iter()
                .filter(|c| c.to_lowercase().contains(&self.search_string))
                .collect(),
            View::Cities => self
                .cities
                .clone()
                .into_iter()
                .filter(|c| c.to_lowercase().contains(&self.search_string))
                .collect(),
            _ => Vec::new(),
        };
        for (i, country) in l.iter().enumerate() {
            let style = match self.view_mode {
                View::Countries => {
                    if i == self.country_index {
                        Style::default().fg(self.config.colors.items_selected)
                    } else {
                        Style::default().fg(self.config.colors.items)
                    }
                }
                View::Cities => {
                    if i == self.city_index {
                        Style::default().fg(self.config.colors.items_selected)
                    } else {
                        Style::default().fg(self.config.colors.items)
                    }
                }
                _ => Style::default().fg(self.config.colors.items),
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

    fn draw_connection(&mut self, f: &mut Frame, _area: Rect, block: Block) {
        let text = Line::from(Span::from(self.connection_output.to_string()))
            .alignment(Alignment::Center)
            .style(Style::default().fg(self.config.colors.connection_output));

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
                self.search_string.clear();
                self.view_mode = match self.view_mode {
                    View::Countries => {
                        self.state.select(Some(0));
                        self.set_cities()?;
                        self.city_index = 0;
                        View::Cities
                    }
                    View::Cities => self.connect()?,
                    View::Connection => {
                        self.country_index = 0;
                        self.city_index = 0;
                        self.set_countries()?;
                        View::Countries
                    }
                };
            }
            event::KeyCode::Down | event::KeyCode::Char('j') => self.increment_index(),
            event::KeyCode::Up | event::KeyCode::Char('k') => self.decrement_index(),
            event::KeyCode::Char('G') => match self.view_mode {
                View::Countries => {
                    self.country_index = self.countries.len() - 1;
                    self.state.select(Some(self.country_index));
                }
                View::Cities => {
                    self.city_index = self.cities.len() - 1;
                    self.state.select(Some(self.city_index))
                }
                _ => {}
            },
            //TODO: fix `gg` keybind
            event::KeyCode::Char('g') => {
                if self.awaiting_second_g {
                    match self.view_mode {
                        View::Countries => {
                            self.country_index = 0;
                            self.state.select(Some(self.country_index));
                        }
                        View::Cities => {
                            self.city_index = 0;
                            self.state.select(Some(self.city_index));
                        }
                        _ => {}
                    }
                    self.awaiting_second_g = false;
                } else {
                    self.awaiting_second_g = true;
                }
            }
            event::KeyCode::Char('/') | event::KeyCode::Char('i') => {
                self.input_mode = InputMode::Search;
            }
            event::KeyCode::Char('h') => match self.view_mode {
                View::Cities => {
                    self.set_countries()?;
                    self.city_index = 0;
                    self.view_mode = View::Countries;
                }
                View::Connection => {
                    self.set_cities()?;
                    self.city_index = 0;
                    self.view_mode = View::Cities;
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn handle_search_mode(&mut self, event: KeyEvent) -> Result<(), AppError> {
        match event.code {
            event::KeyCode::Enter => {
                self.input_mode = InputMode::Normal;
                self.view_mode = match self.view_mode {
                    View::Countries => {
                        self.set_cities()?;
                        self.search_string.clear();
                        View::Cities
                    }
                    View::Cities => View::Cities, // self.connect()?,
                    View::Connection => {
                        self.country_index = 0;
                        View::Countries
                    }
                };
            }
            event::KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
            }
            event::KeyCode::Char(c) => {
                self.search_string.push(c);
                match self.view_mode {
                    View::Countries => {
                        self.countries = self
                            .countries
                            .clone()
                            .into_iter()
                            .filter(|c| c.to_lowercase().contains(&self.search_string))
                            .collect();
                    }
                    View::Cities => {
                        self.cities = self
                            .cities
                            .clone()
                            .into_iter()
                            .filter(|c| c.to_lowercase().contains(&self.search_string))
                            .collect();
                    }
                    View::Connection => {}
                }
                self.country_index = 0;
            }
            event::KeyCode::Backspace => {
                self.search_string.pop();
                self.set_countries()?;
                self.country_index = 0;
            }
            _ => {}
        }
        Ok(())
    }

    fn decrement_country(&mut self) {
        if self.country_index > 0 {
            self.country_index -= 1;
        }
        self.state.select(Some(self.country_index));
    }

    fn increment_country(&mut self) {
        if self.country_index < self.countries.len() - 1 {
            self.country_index += 1;
        }
        self.state.select(Some(self.country_index));
    }

    fn decrement_city(&mut self) {
        if self.city_index > 0 {
            self.city_index -= 1;
        }
        self.state.select(Some(self.city_index));
    }

    fn increment_city(&mut self) {
        if self.city_index < self.cities.len() - 1 {
            self.city_index += 1;
        }
        self.state.select(Some(self.city_index));
    }

    fn decrement_index(&mut self) {
        match self.view_mode {
            View::Countries => self.decrement_country(),
            View::Cities => self.decrement_city(),
            _ => {}
        }
    }

    fn increment_index(&mut self) {
        match self.view_mode {
            View::Countries => self.increment_country(),
            View::Cities => self.increment_city(),
            _ => {}
        }
    }
}
