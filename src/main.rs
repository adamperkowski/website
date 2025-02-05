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
mod data;
mod sse;
mod static_files;
mod templates;

use templates::TEMPLATES;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    std::env::set_var(
        "GITHUB_SECRET",
        secrets.get("GITHUB_SECRET").expect("GITHUB_SECRET not set"),
    );

    let sse_state = sse::init();

    let mut tera_ctx = Context::new();
    tera_ctx.insert("author", "Adam Perkowski");
    tera_ctx.insert("keywords", "Adam Perkowski, adamperkowski, xx0a_q, xeqo, jule, rust, personal, tech, programming, developer, github, adas1per@protonmail.com");
    tera_ctx.insert("canonical", "https://adamperkowski.dev");
    tera_ctx.insert("repo", "https://github.com/adamperkowski/website");

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
        .layer(middleware::from_fn(check_url))
        .route("/projects", get(projects))
        .route("/donate", get(render("donate", &tera_ctx)))
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

async fn check_url(
    req: Request<Body>,
    next: middleware::Next,
) -> Result<Response<Body>, StatusCode> {
    let host = req
        .headers()
        .get("host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default();

    #[cfg(debug_assertions)]
    let expected_host = "127.0.0.1:8000";
    #[cfg(not(debug_assertions))]
    let expected_host = "adamperkowski.dev";

    if host != expected_host {
        let uri = Uri::builder()
            .scheme("https")
            .authority(expected_host)
            .path_and_query(
                req.uri()
                    .path_and_query()
                    .map(|pq| pq.as_str())
                    .unwrap_or("/"),
            )
            .build()
            .expect("failed to build URI")
            .to_string();

        return Ok(Redirect::permanent(&uri).into_response());
    }

    Ok(next.run(req).await)
}
