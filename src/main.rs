use app::App;
use clap::Parser;
use cli::Cli;
use dotenv::dotenv;
use error::AppError;

mod cli;
mod app;
mod data;
mod error;
mod config;

#[cfg(test)]
mod tests;

fn main() -> Result<(), AppError> {
    dotenv().ok();
     
    let args = Cli::parse();
    let mut terminal = ratatui::init();
    let mut app = App::init(args.config).unwrap_or_else(|e| {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    });

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
