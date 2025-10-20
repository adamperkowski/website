use axum::{
    body::Body,
    http::{header, Request, Response, StatusCode, Uri},
    middleware,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tera::Context;

mod api;
mod constants;
mod data;
mod sse;
mod static_files;
mod templates;

use templates::TEMPLATES;

/// The main entry point for the server.
#[shuttle_runtime::main]
async fn axum(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    std::env::set_var(
        constants::GITHUB_SECRET_VAR,
        secrets.get(constants::GITHUB_SECRET_VAR).unwrap(),
    );

    let sse_state = sse::init();

    let mut tera_ctx = Context::new();
    tera_ctx.insert("author", constants::AUTHOR);
    tera_ctx.insert("keywords", constants::KEYWORDS);
    tera_ctx.insert("canonical", &format!("https://{}", constants::HOST));
    tera_ctx.insert("repo", constants::REPO);

    // TODO: might want to look into automating the router & sitemap generation

    let projects = {
        let mut ctx = tera_ctx.clone();
        ctx.insert("projects", &data::projects::PROJECTS);
        render("projects", &ctx)
    };

    let api_routes = Router::new()
        .route("/sse", get(sse::handler))
        .route("/github", post(api::github::handler))
        .fallback(get(api::fallback))
        .with_state(Arc::new(sse_state));

    let app = Router::new()
        .route("/", get(render("home", &tera_ctx)))
        .route("/projects", get(projects))
        .route("/donate", get(render("donate", &tera_ctx)))
        .route("/legal", get(render("legal", &tera_ctx)))
        .nest("/api", api_routes)
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
        .route(
            "/img/badges/mimi-the-car.gif",
            get((
                [(header::CONTENT_TYPE, "image/gif")],
                include_bytes!("../static/mimi-the-car.gif"),
            )),
        )
        .route(
            "/img/mimi-banner.gif",
            get((
                [(header::CONTENT_TYPE, "image/gif")],
                include_bytes!("../static/mimi-banner.gif"),
            )),
        )
        .route(
            "/sitemap.xml",
            get((
                [(header::CONTENT_TYPE, "application/xml")],
                include_str!("../static/sitemap.xml"),
            )),
        )
        .fallback((StatusCode::NOT_FOUND, render("error", &tera_ctx)));

    Ok(app.into())
}

fn render(page: &str, ctx: &Context) -> Html<String> {
    let path = format!("pages/{page}.tera");

    Html(TEMPLATES.render(&path, ctx).unwrap())
}
