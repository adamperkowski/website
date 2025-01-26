use axum::{
    http::header,
    response::{Html, Redirect},
    routing::get,
    Router,
};
use std::{env, net::SocketAddr};
use tera::Context;
use tokio::net::TcpListener;

mod static_files;
mod templates;

use templates::TEMPLATES;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = SocketAddr::from(([127, 0, 0, 1], port.parse().unwrap()));

    println!("http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    let mut tera_ctx = Context::new();
    tera_ctx.insert("title", "Adam Perkowski");
    tera_ctx.insert("author", "Adam Perkowski");
    tera_ctx.insert("description", "Just a random developer...");
    tera_ctx.insert("keywords", "");
    tera_ctx.insert("canonical", "https://adamperkowski.dev");

    let app = Router::new()
        .route("/", get(render("home", &tera_ctx)))
        .route("/static/{*file}", get(static_files::handler))
        .route(
            "/robots.txt",
            get((
                [(header::CONTENT_TYPE, "text/plain")],
                include_str!("../static/robots.txt"),
            )),
        )
        .route(
            "/humans.txt",
            get((
                [(header::CONTENT_TYPE, "text/plain")],
                include_str!("../static/humans.txt"),
            )),
        )
        .route(
            "/favicon.ico",
            get((
                [(header::CONTENT_TYPE, "image/x-icon")],
                include_bytes!("../static/favicon.ico"),
            )),
        )
        // .route("/sitemap.xml", get(([(header::CONTENT_TYPE, "application/xml")], include_str!("../static/sitemap.xml"))))
        .fallback(get(Redirect::temporary("/error")));

    axum::serve(listener, app).await.unwrap();
}

fn render(page: &str, ctx: &Context) -> Html<String> {
    let path = format!("pages/{page}.tera");

    Html(TEMPLATES.render(&path, ctx).unwrap())
}
