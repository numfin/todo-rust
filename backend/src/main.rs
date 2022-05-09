use actix_web::web::ServiceConfig;
use actix_web::{web, App, HttpRequest, HttpServer, Scope};

static PORT: i32 = 3000;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().configure(init_resources));
    server.bind(format!("0.0.0.0:{PORT}"))?.run().await
}

fn init_resources(cfg: &mut ServiceConfig) {
    cfg.service(hello_world_scope());
}

fn hello_world_scope() -> Scope {
    let hello_world_resource = web::resource("/").route(web::get().to(hello_world));
    let hello_world_resource_a = web::resource("/a").route(web::get().to(hello_world));
    web::scope("")
        .service(hello_world_resource)
        .service(hello_world_resource_a)
}

async fn hello_world(req: HttpRequest) -> &'static str {
    println!("{}", req.path());
    "Hello World"
}
