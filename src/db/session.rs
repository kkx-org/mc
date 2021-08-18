use sqlx::{postgres::PgQueryResult, Error, FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
}

impl Session {
    pub async fn get_by_id<'a>(id: Uuid, db: &PgPool) -> Result<Session, Error> {
        sqlx::query_as::<_, Session>("SELECT * FROM active_sessions WHERE id = $1")
            .bind(&id)
            .fetch_one(&*db)
            .await
    }
    pub async fn get_all_by_user_id<'a>(user_id: Uuid, db: &PgPool) -> Result<Vec<Session>, Error> {
        sqlx::query_as::<_, Session>("SELECT * FROM active_sessions WHERE user_id = $1")
            .bind(&user_id)
            .fetch_all(&*db)
            .await
    }
    pub async fn create(name: String, user_id: Uuid, db: &PgPool) -> Result<Session, Error> {
        sqlx::query_as::<_, Session>(
            "INSERT INTO sessions(name, user_id) VALUES ($1, $2) RETURNS *",
        )
        .bind(&name)
        .bind(&user_id)
        .fetch_one(&*db)
        .await
    }
    pub async fn delete_by_id(id: Uuid, db: &PgPool) -> Result<PgQueryResult, Error> {
        sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(&id)
            .execute(&*db)
            .await
    }
    pub async fn delete_all_other_by_user_id(
        user_id: Uuid,
        id: Uuid,
        db: &PgPool,
    ) -> Result<PgQueryResult, Error> {
        sqlx::query("DELETE FROM sessions WHERE (user_id = $1) AND (id <> $2)")
            .bind(&user_id)
            .bind(&id)
            .execute(&*db)
            .await
    }
}
