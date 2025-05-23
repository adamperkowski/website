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
        .layer(middleware::from_fn(check_url))
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

/// Middleware to check the URL client is accessing and redirect if necessary.
/// This ensures the only way to access the site is through the official domain.
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
    let expected_host = constants::HOST;

    if host == expected_host {
        return Ok(next.run(req).await);
    }

    let p_and_q = req
        .uri()
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("/");
    let uri = Uri::builder()
        .scheme("https")
        .authority(expected_host)
        .path_and_query(p_and_q)
        .build()
        .expect("failed to build URI")
        .to_string();

    Ok(Redirect::permanent(&uri).into_response())
}
