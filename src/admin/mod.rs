pub use nickel::{
    Nickel,
    Mount,
    HttpRouter,
    Router,
    Middleware,
    MiddlewareResult,
    Request,
    Response
};
pub use config::{Config};
use middleware::render;
use templates;
use tera::{Context};
use middleware::TEMPLATES;
mod handlers;

fn handler<'mw>(_req: &mut Request<Config>, res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    render(res, |o| templates::hello(o))
//    let config = req.server_data();
//    res.send(format!("Server port: {}", config.server.port))
}

pub fn routers() -> Router<Config> {
    let mut router = Router::new();
    router.get("/", handlers::main::get_main);
    router.get("/pages", middleware! {
        "admin/pages"
    });
    router.get("/tera", middleware! {
        TEMPLATES.render("hello.html", Context::new()).unwrap()
    });
    router.get("/config", handler);
    router
}
