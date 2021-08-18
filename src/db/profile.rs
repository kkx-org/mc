use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Profile {
    pub id: Option<Uuid>,
    pub email: String,
    pub hashed_password: Option<String>,
}

impl Profile {
    pub fn new(email: String) -> User {
        User {
            id: None,
            email,
            hashed_password: None,
        }
    }

    pub async fn get_by_id<'a>(id: Uuid, db: &PgPool) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM profiles WHERE id = $1")
            .bind(&id)
            .fetch_one(&*db)
            .await
    }

    pub async fn save(&mut self, db: &PgPool) -> Result<User, sqlx::Error> {
        self.email = self.email.to_lowercase();
        match &self.id {
            Some(id) => {
                sqlx::query_as(
                    "UPDATE users SET email = $1, hashed_password = $2 WHERE id = $3 RETURNING *",
                )
                    .bind(&self.email)
                    .bind(&self.hashed_password)
                    .bind(id)
                    .fetch_one(&*db)
                .await
            },
            None => {
                sqlx::query_as(
                    "INSERT INTO users(email, hashed_password) VALUES ($1, $2) RETURNING *",
                )
                    .bind(&self.email)
                    .bind(&self.hashed_password)
                    .fetch_one(&*db)
                .await
            }
        }
    }
}

