use clap::Clap;

#[derive(Debug, Clap)]
#[clap(version=env!("CARGO_PKG_VERSION"), author=env!("CARGO_PKG_AUTHORS"))]
pub struct Config {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clap)]
pub enum Command {
    Serve(Serve),
}

#[derive(Debug, Clap)]
pub struct Serve {
    #[clap(short, default_value = "5000", env = "PORT")]
    pub port: u16,

    #[clap(short, default_value = "0.0.0.0", env = "HOST")]
    pub host: String,

    // NOTE: rwc -> read/write/create if not exist
    #[clap(
        short,
        long,
        default_value = "sqlite:./objstor.db?mode=rwc",
        env = "CONNECTION_STRING"
    )]
    pub connection_string: String,

    #[clap(short, long, default_value = "CHANGEME-OBJSTOR", env = "SALT")]
    pub salt: String,
}

pub fn parse() -> Config {
    Config::parse()
}
