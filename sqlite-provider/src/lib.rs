use async_trait::async_trait;
use objstor::{hash_with_salt, new_id, ObjstorError, ObjstorProvider, User, UserObjstorProvider};
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};

#[derive(Clone, Debug)]
pub struct SqliteObjstorProvider {
    pool: SqlitePool,
    salt: String,
}

impl SqliteObjstorProvider {
    pub fn new(pool: SqlitePool, salt: &str) -> Self {
        Self {
            pool,
            salt: salt.to_owned(),
        }
    }

    pub async fn connect(connection_string: &str, salt: &str) -> Result<Self, ObjstorError> {
        Ok(Self::new(
            SqlitePool::connect(connection_string)
                .await
                .map_err(|e| ObjstorError::ConnectionError(e.to_string()))?,
            salt,
        ))
    }
}

#[async_trait]
impl ObjstorProvider for SqliteObjstorProvider {
    async fn init(&self) -> Result<(), ObjstorError> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| ObjstorError::ProviderMigrationError(e.to_string()))?;

        if !self.has_users().await? {
            self.create_user(&User {
                id: new_id(),
                username: String::from("admin"),
                password: Some(String::from("admin")),
                is_locked: false,
                is_admin: true,
            })
            .await?;
        }

        Ok(())
    }
}

#[async_trait]
impl UserObjstorProvider for SqliteObjstorProvider {
    async fn has_users(&self) -> Result<bool, ObjstorError> {
        let user_count: i64 = sqlx::query_scalar("SELECT count(*) FROM user")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| ObjstorError::ConnectionError(e.to_string()))?;
        Ok(user_count != 0)
    }

    async fn create_user(&self, user: &User) -> Result<String, ObjstorError> {
        if user.id.is_empty() {
            return Err(ObjstorError::InvalidUserId(user.id.clone()));
        }

        if user.password.is_none() {
            return Err(ObjstorError::EmptyPassword());
        }

        sqlx::query(
            r#"
                INSERT INTO user
                    (id, username, password, is_admin, is_locked)
                    VALUES
                    (?, ?, ?, ?, ?);
            "#,
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(hash_with_salt(&user.password.clone().unwrap(), &self.salt)?)
        .bind(&user.is_admin)
        .bind(&user.is_locked)
        .execute(&self.pool)
        .await
        .map_err(|e| ObjstorError::ConnectionError(e.to_string()))?;

        Ok(user.id.clone())
    }

    async fn validate_user(&self, username: &str, password: &str) -> Result<bool, ObjstorError> {
        let count: i32 = sqlx::query_scalar(
            "SELECT count(*) FROM user WHERE is_locked=0 and username=? and password=?",
        )
        .bind(username)
        .bind(hash_with_salt(password, &self.salt)?)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ObjstorError::ConnectionError(e.to_string()))?;
        Ok(count == 1)
    }

    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, ObjstorError> {
        let user: Option<User> = sqlx::query("SELECT * from user where username=?")
            .bind(username)
            .map(|row: SqliteRow| User {
                username: row.get("username"),
                id: row.get("id"),
                password: None,
                is_locked: row.get("is_locked"),
                is_admin: row.get("is_admin"),
            })
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ObjstorError::ConnectionError(e.to_string()))?;
        Ok(user)
    }
}
