use anyhow::Result;
use std::env;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct Config {
    addr: String,
    secret: String,
    conn_str: String,
}

impl Config {
    pub fn new_from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let port = env::var("PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()?;

        let addr = format!("0.0.0.0:{}", port);

        // TODO: generate random secret
        let secret = env::var("SECRET").unwrap_or_else(|_| String::from("CHANGEME-OBJSTOR"));

        if secret == "CHANGEME-OBJSTOR" {
            println!(
                "You are using the default objstor secret. Please change for increased security."
            );
        }

        // NOTE: rwc -> read/write/create if not exist
        let conn_str = env::var("CONNECTION_STRING")
            .unwrap_or_else(|_| String::from("sqlite:./objstor.db?mode=rwc"));

        Ok(Config {
            addr,
            secret,
            conn_str,
        })
    }

    pub fn get_addr(&self) -> Result<SocketAddr> {
        let addr: SocketAddr = self.addr.parse()?;
        Ok(addr)
    }

    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    pub fn get_conn_str(&self) -> &str {
        &self.conn_str
    }
}
