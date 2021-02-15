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
}

pub fn parse() -> AppOpt {
    AppOpt::parse()
}
