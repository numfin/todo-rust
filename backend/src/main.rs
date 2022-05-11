use actix_web::middleware::Logger;
use actix_web::web::{self, ServiceConfig};
use actix_web::{App, HttpServer};
use db_pool::DBPool;
use env_logger::Env;

mod db_pool;
mod page;
mod todo;

static PORT: i32 = 3000;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app_data = web::Data::new(DBPool::connect().await);
    let server = HttpServer::new(move || {
        App::new()
            .configure(init_resources)
            .app_data(app_data.clone())
            .wrap(Logger::default())
    });
    server.bind(format!("0.0.0.0:{PORT}"))?.run().await
}

fn init_resources(cfg: &mut ServiceConfig) {
    cfg.service(todo::scope());
}
