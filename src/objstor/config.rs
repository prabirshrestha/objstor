use anyhow::{bail, Result};
use std::net::SocketAddr;
use std::{env, sync::Arc};

use super::{new_sqlite_user_backend, UserBackend};

pub struct Config {
    addr: String,
    blob_conn_str: String,
    db_conn_str: String,
    root_url: String,
    user_backend: Box<dyn UserBackend>,
}

impl Config {
    pub fn new_from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let port = env::var("PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()?;

        let addr = format!("0.0.0.0:{}", port);

        let blob_conn_str = env::var("BLOB_CONNECTION_STRING")
            .unwrap_or_else(|_| String::from("file://./data/blobs"));

        let db_conn_str = env::var("DB_CONNECTION_STRING")
            .unwrap_or_else(|_| String::from("sqlite:./data/data.db"));

        let user_backend = if db_conn_str.starts_with("sqlite:") {
            Box::new(new_sqlite_user_backend())
        } else {
            bail!("connection string not supported");
        };

        let root_url = env::var("ROOT_URL").unwrap_or_else(|_| addr.clone());

        Ok(Config {
            addr,
            blob_conn_str,
            db_conn_str,
            root_url,
            user_backend,
        })
    }

    pub fn get_addr(&self) -> Result<SocketAddr> {
        let addr: SocketAddr = self.addr.parse()?;
        Ok(addr)
    }

    pub fn db_conn_str(&self) -> &str {
        &self.db_conn_str
    }

    pub fn blob_conn_str(&self) -> &str {
        &self.blob_conn_str
    }

    pub fn root_url(&self) -> &str {
        &self.root_url
    }
}
