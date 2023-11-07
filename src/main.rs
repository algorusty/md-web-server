mod generate_navbar;
mod routes;
mod serve_markdown;
mod html_utils;

use actix_web::{App, HttpServer};
use routes::configure_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().configure(configure_app));

    // Bind to an address and start the server, akin to app.listen() in Express.js
    println!("Server running at http://127.0.0.1:8080/");
    server.bind("127.0.0.1:8080")?.run().await
}
