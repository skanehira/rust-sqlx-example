use anyhow::Result;
use sqlx::{Executor, MySql, MySqlPool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateUserInput {
    pub id: String,
    pub name: String,
    pub email: String,
}

pub trait Repository {
    async fn create(&self, input: CreateUserInput) -> Result<()>;
}

pub struct UserManagementRepository<E> {
    db: E,
}

impl<E> UserManagementRepository<E> {
    pub fn new(db: E) -> Self {
        Self { db }
    }
}

impl<E> Repository for UserManagementRepository<E>
where
    E: for<'c> Executor<'c, Database = MySql> + Clone + Send + Sync,
{
    async fn create(&self, input: CreateUserInput) -> Result<()> {
        let query = sqlx::query!(
            r#"
            INSERT INTO users (id, name, email)
            VALUES (?, ?, ?)
            "#,
            input.id,
            input.name,
            input.email,
        );

        query.execute(self.db).await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = MySqlPool::connect("mysql://sandbox:sandbox@localhost:3307/sandbox").await?;

    let input = CreateUserInput {
        id: "1".into(),
        name: "John Doe".into(),
        email: "".into(),
    };

    // non transactional
    UserManagementRepository::new(&pool.clone())
        .create(input.clone())
        .await?;

    // transactional
    let tx = pool.begin().await?;
    UserManagementRepository::new(&mut tx).create(input).await?;
    tx.commit().await?;

    Ok(())
}
