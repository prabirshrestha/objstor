use clap::Clap;

#[derive(Debug, Clap)]
#[clap(version=env!("CARGO_PKG_VERSION"), author=env!("CARGO_PKG_AUTHORS"))]
pub struct Config {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clap)]
pub enum Command {}

pub fn parse() -> Config {
    Config::parse()
}
