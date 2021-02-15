use clap::Clap;

#[derive(Debug, Clap)]
#[clap(version=env!("CARGO_PKG_VERSION"), author=env!("CARGO_PKG_AUTHORS"))]
pub struct AppOpt {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clap)]
pub enum Command {
    Serve(Serve),
}

#[derive(Debug, Clap)]
pub struct Serve {
    #[clap(short, default_value = "3000", env = "PORT")]
    pub port: u64,

    // NOTE: rwc -> read/write/create if not exist
    #[clap(
        short,
        long,
        default_value = "sqlite:./objstor.db?mode=rwc",
        env = "CONNECTION_STRING"
    )]
    pub connection_string: String,
}

pub fn parse() -> AppOpt {
    AppOpt::parse()
}
