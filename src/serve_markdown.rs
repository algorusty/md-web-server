use actix_web::{web, HttpResponse, Responder}; 
use pulldown_cmark::{html, Options, Parser};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use crate::html_utils::html_header;
use crate::generate_navbar::generate_navbar;

// Serve markdown files as HTML from the 'public' directory
pub async fn serve_markdown(path: web::Path<PathBuf>) -> impl Responder {
    let file_path = PathBuf::from("public").join(&*path).canonicalize().unwrap_or_default();

    // Check if the requested file is a markdown file and prevent directory traversal
    if file_path.is_file() && file_path.extension().map_or(true, |ext| ext != "md") {
        return HttpResponse::BadRequest().body("Not a markdown file");
    }

    // Attempt to read the markdown file content
    let markdown_content = match read_to_string(&file_path) {
        Ok(content) => content,
        Err(_) => return HttpResponse::NotFound().body("File not found"),
    };

    // Generate the navbar HTML
    let navbar_html = generate_navbar(&Path::new("public")).await; // Call this only once

    // Generate the HTML for the markdown content
    let parser = Parser::new_ext(&markdown_content, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Construct the full HTML response
    let full_html = format!(
        r#"{}<div class="sidebar">{}</div><main class="content"><article class="markdown-body">{}</article></main></body></html>"#,
        html_header(),
        navbar_html, // Reuse the navbar_html variable here
        html_output
    );

    // Respond with the generated HTML
    HttpResponse::Ok().content_type("text/html").body(full_html)
}