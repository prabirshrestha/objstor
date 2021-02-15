use super::{
    opt::{AppOpt, Command},
    server::serve,
};
use anyhow::Result;

pub async fn run(opt: &AppOpt) -> Result<()> {
    match &opt.command {
        Command::Serve(s) => serve(&s).await?,
    }
    Ok(())
}
