use argon2::{
    password_hash::{
        Error::PhcStringTooShort, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use rand_core::OsRng;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Option<Uuid>,
    pub email: String,
    pub hashed_password: Option<String>,
}

impl User {
    pub fn new(email: String) -> User {
        User {
            id: None,
            email,
            hashed_password: None,
        }
    }

    pub fn set_password(&mut self, password: String) -> Result<(), argon2::password_hash::Error> {
        if password.len() < 8 {
            return Err(argon2::password_hash::Error::PhcStringTooShort);
        };

        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        self.hashed_password = Some(
            argon2
                .hash_password_simple(password.as_bytes(), salt.as_ref())?
                .to_string(),
        );

        Ok(())
    }
    pub fn verify_password(&self, password: String) -> Result<(), argon2::password_hash::Error> {
        if self.hashed_password.is_none() {
            return Err(PhcStringTooShort);
        }

        let argon2 = Argon2::default();

        let hashed_password = self.hashed_password.as_ref().unwrap();
        let parsed_hash = PasswordHash::new(&hashed_password)?;

        argon2.verify_password(password.as_bytes(), &parsed_hash)?;

        Ok(())
    }

    pub async fn get_by_id<'a>(id: Uuid, db: &PgPool) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(&id)
            .fetch_one(&*db)
            .await
    }
    pub async fn get_by_email<'a>(email: String, db: &PgPool) -> Result<User, sqlx::Error> {
        let email = email.to_lowercase();
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(&email)
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
            }
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
