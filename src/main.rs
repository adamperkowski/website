use axum::Router;

mod metadata;

use metadata::{ChangeFreq, RobotsTXT, Sitemap, Uri};

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    Ok(build_routes().into())
}

fn build_routes() -> Router {
    let uris = &[Uri::new(
        "/",
        "home",
        true,
        Some(ChangeFreq::Monthly),
        Some(1.0),
    )];

    let sitemap = Sitemap::from_uris(uris);
    let robots = RobotsTXT::from_uris(uris);

    Router::new()
}
