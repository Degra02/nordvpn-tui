
#[test]
fn load_config() {
    dotenv::dotenv().ok();
    // let cli = <crate::cli::Cli as clap::Parser>::parse();
    let config = crate::config::Config::load(Some("config.toml")).unwrap();
    println!("{:?}", config);
}
