use anyhow::Result;
use sqlx::MySqlConnection;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateUserInput {
    pub id: String,
    pub name: String,
    pub email: String,
    pub confirmed: bool,
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub trait Repository {
    async fn create(&mut self, input: CreateUserInput) -> Result<()>;
}

pub struct UserManagementRepository<'a> {
    db: &'a mut MySqlConnection,
}

impl<'a> UserManagementRepository<'a> {
    pub fn new(db: &'a mut MySqlConnection) -> Self {
        Self { db }
    }
}

impl Repository for UserManagementRepository<'_> {
    async fn create(&mut self, input: CreateUserInput) -> Result<()> {
        let query = sqlx::query!(
            r#"
            INSERT INTO users (id, name, email, confirmed, birthday, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            input.id,
            input.name,
            input.email,
            input.confirmed,
            input.birthday,
            input.created_at,
            input.updated_at,
        );

        let x = &mut *self.db;
        query.execute(x).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use sqlx::{MySql, MySqlPool, Pool};

    async fn new_database() -> Result<Pool<MySql>> {
        let pool = MySqlPool::connect("mysql://sandbox:sandbox@localhost:3307/sandbox").await?;
        Ok(pool)
    }

    #[tokio::test]
    async fn should_create_user_with_tx() -> Result<()> {
        let pool = new_database().await?;

        let now = chrono::Utc::now();
        let id = uuid::Uuid::new_v4().to_string();
        let email: String = format!("{}@gmail.com", id);
        let name: String = "John Doe".into();
        let birthday = chrono::DateTime::parse_from_rfc3339("1999-10-03T00:00:00Z")?.to_utc();

        let input = CreateUserInput {
            id: id.clone(),
            name: name.clone(),
            email: email.clone(),
            confirmed: false,
            birthday: Some(birthday),
            created_at: now,
            updated_at: now,
        };

        let mut tx = pool.begin().await?;
        let conn = &mut *tx;
        UserManagementRepository::new(conn)
            .create(input.clone())
            .await?;
        tx.commit().await?;
        Ok(())
    }

    #[tokio::test]
    async fn should_create_user() -> Result<()> {
        let pool = new_database().await?;

        let now = chrono::Utc::now();
        let id = uuid::Uuid::new_v4().to_string();
        let email: String = format!("{}@gmail.com", id);
        let name: String = "John Doe".into();
        let birthday = chrono::DateTime::parse_from_rfc3339("1999-10-03T00:00:00Z")?.to_utc();

        let input = CreateUserInput {
            id: id.clone(),
            name: name.clone(),
            email: email.clone(),
            confirmed: false,
            birthday: Some(birthday),
            created_at: now,
            updated_at: now,
        };

        let mut conn = pool.acquire().await?;
        let x = conn.as_mut();
        UserManagementRepository::new(x).create(input).await?;

        Ok(())
    }
}
