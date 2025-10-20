use axum::{
    http::{self, header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

mod constants;
mod data;
mod metadata;
mod templates;

use metadata::{ChangeFreq, RobotsTXT, Sitemap, Uri};
use tera::Context;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    Ok(build_routes().into())
}

fn build_routes() -> Router {
    let uris = &[
        Uri::new("/", "home", true, Some(ChangeFreq::Monthly), Some(1.0)),
        Uri::new(
            "/donate",
            "donate",
            true,
            Some(ChangeFreq::Yearly),
            Some(0.8),
        ),
        Uri::new(
            "/contact",
            "contact",
            true,
            Some(ChangeFreq::Yearly),
            Some(0.8),
        ),
        Uri::new(
            "/projects",
            "construction",
            true,
            Some(ChangeFreq::Monthly),
            Some(0.8),
        ),
    ];

    let sitemap = Sitemap::from_uris(uris).to_string();
    let robots = RobotsTXT::from_uris(uris).to_string();

    let redirect_router: Router = Router::new()
        .route("/favicon.ico", get(|| redirect("/img/favicon.ico")))
        .route("/styles.css", get(|| redirect("/static/styles.css")));

    let mut router = Router::new()
        .route(
            "/sitemap.xml",
            get(([(header::CONTENT_TYPE, "application/xml")], sitemap)),
        )
        .route(
            "/robots.txt",
            get(([(header::CONTENT_TYPE, "text/plain")], robots)),
        )
        .merge(redirect_router)
        .nest_service("/img", ServeDir::new("img"))
        .nest_service("/static", ServeDir::new("static"))
        .route("/healthz", get("hello :3"))
        .fallback(fallback_handler);

    let mut ctx = Context::new();
    ctx.insert("host", constants::HOST);
    ctx.insert("badges", &data::BADGES);
    ctx.insert("uris", uris);

    for uri in uris {
        ctx.insert(
            "canonical",
            format!("{}{}", constants::HOST, uri.uri).trim_end_matches('/'),
        );
        router = router.route(uri.uri, get(render(uri.template, &ctx)));
    }

    router
}

async fn redirect(location: &str) -> Response {
    (
        StatusCode::PERMANENT_REDIRECT,
        [(header::LOCATION, location)],
        "redirecting...",
    )
        .into_response()
}

fn render(page_name: &str, ctx: &Context) -> Html<String> {
    let path = format!("pages/{page_name}.tera");

    match templates::TEMPLATES.render(&path, ctx) {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("failed to render template {page_name}: {err}");
            Html(format!("template error: {err} :c"))
        }
    }
}

async fn fallback_handler(uri: http::Uri) -> Response {
    let path = uri.path();

    if path != "/" && path.ends_with('/') {
        let new_path = path.trim_end_matches('/');
        let new_loc = format!("{}{}", constants::HOST, new_path);

        return redirect(&new_loc).await;
    }

    (StatusCode::NOT_FOUND, "not nound :c").into_response()
}
