use app::App;
use clap::Parser;
use cli::Cli;
use error::AppError;

mod cli;
mod app;
mod data;
mod error;

fn main() -> Result<(), AppError> {
    let _args = Cli::parse();
    let mut terminal = ratatui::init();
    let mut app = App::init().unwrap_or_else(|e| {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    });

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
