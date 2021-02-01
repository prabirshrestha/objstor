use anyhow::Result;
use std::env;
use std::net::SocketAddr;

pub struct Config {
    addr: String,
    secret: String,
}

impl Config {
    pub fn new_from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let port = env::var("PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()?;

        let addr = format!("0.0.0.0:{}", port);

        // TODO: generate random secret
        let secret = env::var("SECRET").unwrap_or_else(|_| String::from("objstorsecret"));

        let var_name = Ok(Config { addr, secret });
        var_name
    }

    pub fn get_addr(&self) -> Result<SocketAddr> {
        let addr: SocketAddr = self.addr.parse()?;
        Ok(addr)
    }

    pub fn get_secret(&self) -> &str {
        &self.secret
    }
}
