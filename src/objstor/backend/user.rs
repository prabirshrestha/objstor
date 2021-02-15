/*
use crate::objstor::{hash_with_salt, uuid, Config, User, UserBackend};
use anyhow::{bail, Result};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Executor, SqlitePool};

pub struct SqliteUserBackend {
    config: Box<Config>,
    pool: Box<SqlitePool>,
}

impl SqliteUserBackend {
    pub fn new(config: Box<Config>, pool: Box<SqlitePool>) -> Self {
        SqliteUserBackend { config, pool }
    }
}

#[async_trait]
impl UserBackend for SqliteUserBackend {
    async fn init(&self) -> Result<()> {
        // TODO: add more index
        let mut conn = self.pool.acquire().await?;
        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS user (
                id varchar(256) PRIMARY KEY,
                username varchar(256) UNIQUE NOT NULL,
                password varchar(256) NOT NULL,
                created DATETIME NOT NULL,
                locked BOOLEAN NOT NULL CHECK (locked IN (0,1)),
                is_admin BOOLEAN NOT NULL CHECK (is_admin IN (0,1))
            )"#,
        )
        .await?;

        let user_count: i64 = sqlx::query_scalar("SELECT count(*) FROM user")
            .fetch_one(&mut conn)
            .await?;

        if user_count == 0 {
            // create admin user
            self.create_user(&User {
                id: uuid()?,
                username: String::from("admin"),
                password: Some(String::from("admin")),
                created: Utc::now(),
                locked: false,
                is_admin: true,
            })
            .await?;

            println!(
                "Default admin user created. Please change the password for increased security"
            );
        }

        // type: osfs, s3
        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS storage (
                id varchar(256) PRIMARY KEY,
                name varchar(500) NOT NULL,
                created DATETIME NOT NULL,
                type varchar(256) NOT NULL,
                data TEXT NOT NULL,
                indexed BOOLEAN NOT NULL CHECK (indexed IN (0,1))
            )"#,
        )
        .await?;

        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS object (
                id varchar(256) PRIMARY KEY,
                parent_id varchar(256),
                storage_id varchar(256) NOT NULL,
                name TEXT NOT NULL,
                type ObjectType NOT NULL CHECK (type IN (0,1)), -- 0:file, 1:dir
                ctime DATETIME NOT NULL,
                mtime DATETIME NOT NULL,
                md5 varchar(32),
                sha256 varchar(256),
                description TEXT,                               -- full text search
                data JSON,                                      -- extra metadata
                FOREIGN KEY (parent_id) REFERENCES objects (id),
                FOREIGN KEY (storage_id) REFERENCES storage (id)
            )"#,
        )
        .await?;

        Ok(())
    }

    async fn create_user(&self, user: &User) -> Result<String> {
        let mut conn = self.pool.acquire().await?;
        if user.password.is_none() {
            bail!("password required");
        }
        let password = user.password.clone().unwrap();
        sqlx::query(
            r#"INSERT INTO user
                    (id, username, password, created, locked, is_admin)
                    VALUES
                    (?, ?, ?, ?, ?, ?);
            "#,
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(hash_with_salt(&password, self.config.get_secret())?)
        .bind(user.created)
        .bind(user.locked)
        .bind(user.is_admin)
        .execute(&mut conn)
        .await?;
        Ok(user.id.clone())
    }

    async fn validate_password(&self, username: &str, password: &str) -> Result<bool> {
        let mut conn = self.pool.acquire().await?;
        let count: i32 = sqlx::query_scalar(
            "SELECT count(*) FROM user WHERE locked=0 and username=? and password=?",
        )
        .bind(username)
        .bind(hash_with_salt(password, self.config.get_secret())?)
        .fetch_one(&mut conn)
        .await?;

        Ok(count == 1)
    }
}
*/
