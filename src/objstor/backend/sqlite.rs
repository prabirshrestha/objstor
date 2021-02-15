use super::{ObjstorBackend, User, UserBackend};
use crate::objstor::utils::{hash_with_salt, uuid};
use anyhow::{bail, Result};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;

#[derive(Clone, Debug)]
pub struct SqliteObjstorBackend {
    pool: SqlitePool,
    salt: String,
}

impl SqliteObjstorBackend {
    pub async fn new(connection_string: &str, salt: String) -> Result<Self> {
        Ok(SqliteObjstorBackend {
            pool: SqlitePool::connect(connection_string).await?,
            salt,
        })
    }
}

#[async_trait]
impl ObjstorBackend for SqliteObjstorBackend {
    async fn init(&self) -> Result<()> {
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS user (
                id varchar(256) PRIMARY KEY,
                username varchar(256) UNIQUE NOT NULL,
                password varchar(256) NOT NULL,
                created DATETIME NOT NULL,
                is_locked BOOLEAN NOT NULL CHECK (is_locked IN (0,1)),
                is_admin BOOLEAN NOT NULL CHECK (is_admin IN (0,1))
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        let user_count: i64 = sqlx::query_scalar("SELECT count(*) FROM user")
            .fetch_one(&self.pool)
            .await?;

        if user_count == 0 {
            // create admin user
            self.create_user(&User {
                id: uuid()?,
                username: String::from("admin"),
                password: Some(String::from("admin")),
                created: Utc::now(),
                is_locked: false,
                is_admin: true,
            })
            .await?;
        }

        Ok(())
    }
}

#[async_trait]
impl UserBackend for SqliteObjstorBackend {
    async fn create_user(&self, user: &User) -> Result<String> {
        if user.password.is_none() {
            bail!("password required");
        }
        let password = user.password.clone().unwrap();
        sqlx::query(
            r#"INSERT INTO user
                    (id, username, password, created, is_locked, is_admin)
                    VALUES
                    (?, ?, ?, ?, ?, ?);
            "#,
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(hash_with_salt(&password, &self.salt)?)
        .bind(user.created)
        .bind(user.is_locked)
        .bind(user.is_admin)
        .execute(&self.pool)
        .await?;

        Ok(user.id.clone())
    }

    async fn validate_user(&self, username: &str, password: &str) -> Result<bool> {
        let count: i32 = sqlx::query_scalar(
            "SELECT count(*) FROM user WHERE locked=0 and username=? and password=?",
        )
        .bind(username)
        .bind(hash_with_salt(password, &self.salt)?)
        .fetch_one(&self.pool)
        .await?;
        Ok(count == 1)
    }
}
