mod db;
mod error;
mod routes;

use actix_web::{
    middleware::Logger,
    http::StatusCode,
    error::JsonPayloadError,
    web::{Data, FormConfig, JsonConfig, PathConfig, QueryConfig},
    App, FromRequest, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use sqlx::PgPool;
use std::{env, error::Error};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .app_data(JsonConfig::default().error_handler(|err, _req| {
                let err_str = err.to_string();
                actix_web::error::InternalError::from_response(err, HttpResponse::BadRequest().json(error::ApiErrorResponse {
                    error: String::from("invalid_json_payload"),
                    message: err_str
                })).into()
            }))
            .app_data(QueryConfig::default().error_handler(|err, _req| {
                let err_str = err.to_string();
                actix_web::error::InternalError::from_response(err, HttpResponse::BadRequest().json(error::ApiErrorResponse {
                    error: String::from("invalid_query"),
                    message: err_str
                })).into()
            }))
            .app_data(PathConfig::default().error_handler(|err, _req| {
                let err_str = err.to_string();
                actix_web::error::InternalError::from_response(err, HttpResponse::NotFound().json(error::ApiErrorResponse {
                    error: String::from("invalid_path"),
                    message: err_str
                })).into()
            }))
            .app_data(FormConfig::default().error_handler(|err, _req| {
                let err_str = err.to_string();
                actix_web::error::InternalError::from_response(err, HttpResponse::NotFound().json(error::ApiErrorResponse {
                    error: String::from("invalid_form_payload"),
                    message: err_str
                })).into()
            }))
            .wrap(Logger::default())
            .configure(routes::init)
    })
    .bind(env::var("LISTEN_ADDRESS").expect("LISTEN_ADDRESS is not set"))?
    .run()
    .await
}
