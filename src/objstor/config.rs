use anyhow::Result;
use std::env;
use std::net::SocketAddr;

pub struct Config {
    addr: String,
    conn_str: String,
    root_url: String,
}

impl Config {
    pub fn new_from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let port = env::var("PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()?;

        let addr = format!("0.0.0.0:{}", port);

        let conn_str =
            env::var("CONNECTION_STRING").unwrap_or_else(|_| String::from("sqlite:data.db"));

        let root_url = env::var("ROOT_URL").unwrap_or_else(|_| addr.clone());

        Ok(Config {
            addr,
            conn_str,
            root_url,
        })
    }

    pub fn get_addr(&self) -> Result<SocketAddr> {
        let addr: SocketAddr = self.addr.parse()?;
        Ok(addr)
    }

    pub fn conn_str(&self) -> &str {
        &self.conn_str
    }

    pub fn root_url(&self) -> &str {
        &self.root_url
    }
}
