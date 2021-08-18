use crate::db::user::User;
use actix_web::{
    post,
    web::{self, scope, Json},
    HttpResponse, Responder,
};
use sqlx::PgPool;
pub mod error;
pub mod structures;

#[post("register")]
async fn register(
    data: Json<structures::RegisterRequest>,
    db: web::Data<PgPool>,
) -> Result<impl Responder, crate::error::Error> {
    if User::get_by_email(data.email.clone(), &*db).await.is_ok() {
        return Err(error::AuthError::EmailAlreadyRegistered.into());
    }

    let mut user = User::new(data.email.clone());

    if user.set_password(data.password.clone()).is_err() {
        return Err(error::AuthError::ValidationFailed(String::from("password")).into());
    };

    user.save(&*db).await?;

    Ok(HttpResponse::Ok())
}

#[post("login")]
async fn login(data: Json<structures::RegisterRequest>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("account").service(register));
}
