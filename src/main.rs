use std::net::TcpListener;
use zero2prod::startup;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    startup::run(listener)?.await
}
