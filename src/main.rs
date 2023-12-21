use flexi_logger::LevelFilter;
use simple_logger::SimpleLogger;
use warp::Filter;

fn init_logger() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
}

#[tokio::main]
async fn main() {
    init_logger();

    let root = warp::path::end().and(warp::fs::file("static/index.html"));

    let static_folder = warp::path("static")
        .and(warp::fs::dir("./static"));

    let routes = warp::get().and(
        root
        .or(static_folder)
    );

    #[cfg(debug_assertions)]
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;

    #[cfg(not(debug_assertions))]
    warp::serve(routes)
        .tls()
        .cert_path("/etc/letsencrypt/live/hh-lang.de/fullchain.pem")
        .key_path("/etc/letsencrypt/live/hh-lang.de/privkey.pem")
        .run(([0, 0, 0, 0], 443)).await;
}