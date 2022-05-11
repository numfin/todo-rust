use actix_web::{web, Scope};

mod controllers;
mod models;

pub fn scope() -> Scope {
    web::scope("")
        .route("/", web::get().to(controllers::list))
        .route("/", web::post().to(controllers::add))
        .route("/{id}", web::patch().to(controllers::toggle))
        .route("/{id}", web::delete().to(controllers::delete))
}
