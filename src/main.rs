use axum::{
    http::header,
    response::{Html, Redirect},
    routing::get,
    Router,
};
use tera::Context;

mod static_files;
mod templates;

use templates::TEMPLATES;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let mut tera_ctx = Context::new();
    tera_ctx.insert("author", "Adam Perkowski");
    tera_ctx.insert("keywords", "");
    tera_ctx.insert("canonical", "https://adamperkowski.dev");

    let app = Router::new()
        .route("/", get(render("home", &tera_ctx)))
        .route("/error", get(render("error", &tera_ctx)))
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

    Ok(app.into())
}

fn render(page: &str, ctx: &Context) -> Html<String> {
    let path = format!("pages/{page}.tera");

    Html(TEMPLATES.render(&path, ctx).unwrap())
}
