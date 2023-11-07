use crate::serve_markdown::serve_markdown;
use actix_web::{web, HttpResponse, HttpRequest, http};

// Change the return type of the function to HttpResponse.
async fn redirect_to_resume() -> HttpResponse {
    HttpResponse::Found()
        .append_header((http::header::LOCATION, "/Resume.md"))
        .finish()
}

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/{filename:.*}").route(web::get().to(serve_markdown)))
      .route("/", web::get().to(redirect_to_resume)) // Direct root path to redirect_to_resume
      .default_service(
          web::route()
              .to(|req: HttpRequest| {
                  async move {
                      match *req.method() {
                          http::Method::GET => redirect_to_resume().await,
                          _ => HttpResponse::MethodNotAllowed().finish(),
                      }
                  }
              }),
      );
}
