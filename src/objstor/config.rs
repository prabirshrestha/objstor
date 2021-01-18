use anyhow::Result;
use std::env;
use std::net::SocketAddr;

pub struct Config {
    addr: String,
    conn_str: String,
}

impl Config {
    pub fn new_from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let port = env::var("PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()?;

        let addr = format!("0.0.0.0:{}", port);

        let conn_str =
            env::var("CONNECTION_STRING").unwrap_or_else(|_| String::from("sqlite::memory:"));

        Ok(Config { addr, conn_str })
    }

    pub fn get_addr(&self) -> Result<SocketAddr> {
        let addr: SocketAddr = self.addr.parse()?;
        Ok(addr)
    }

    pub fn conn_str(&self) -> &str {
        &self.conn_str
    }
}
