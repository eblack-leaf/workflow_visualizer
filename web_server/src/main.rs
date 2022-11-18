use std::path::Path;

use warp::hyper::header::HeaderName;
use warp::Filter;

fn cross_origin_embedder_policy(reply: impl warp::Reply) -> impl warp::Reply {
    warp::reply::with_header(
        reply,
        HeaderName::from_static("cross-origin-embedder-policy"),
        "require-corp",
    )
}

fn cross_origin_opener_policy(reply: impl warp::Reply) -> impl warp::Reply {
    warp::reply::with_header(
        reply,
        HeaderName::from_static("cross-origin-opener-policy"),
        "same-origin",
    )
}

#[tokio::main]
async fn main() {
    let project_root = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf();
    let _profile = "debug";
    let build_dir = project_root.join("web_app_build");
    let cors = warp::cors().allow_any_origin().allow_methods(vec!["GET"]);
    let routes = warp::fs::dir(build_dir)
        .map(cross_origin_embedder_policy)
        .map(cross_origin_opener_policy)
        .with(cors);
    let cert_dir = project_root.join("web_server").join("ssl_certs");
    warp::serve(routes)
        .tls()
        .key_path(cert_dir.join("key.pem"))
        .cert_path(cert_dir.join("cert.pem"))
        .run(([0, 0, 0, 0], 3030))
        .await;
}
