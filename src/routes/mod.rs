use actix_web::{
    get,
    web::{scope, ServiceConfig},
    HttpResponse, Responder,
};
mod auth;
mod mojang;


pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(scope("mojang").configure(mojang::init));
    auth::init(cfg);
}
