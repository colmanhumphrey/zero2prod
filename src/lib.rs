use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder}; // HttpRequest,

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new().route("/health_check", web::get().to(health_check))
        // .route("/", web::get().to(greet))
        // .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run();
    Ok(server)
}
