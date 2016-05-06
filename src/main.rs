mod routes;

extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron as handlebars;

use std::error::Error;
use std::sync::Arc;
use iron::prelude::*;
use iron::AfterMiddleware;
use mount::Mount;
use staticfile::Static;
use handlebars::{HandlebarsEngine, DirectorySource};

#[cfg(feature = "watch")]
use handlebars::Watchable;

struct ErrorReporter;

impl AfterMiddleware for ErrorReporter {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("{}", err.description());
        Err(err)
    }
}

static TEMPLATES_PATH: &'static str = "./src/templates/";

fn get_handlebars() -> HandlebarsEngine {
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new(TEMPLATES_PATH, ".hbs")));
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }
    hbse
}

fn start(hbse: Arc<HandlebarsEngine>) {
    let mut chain = Chain::new(routes::new());
    chain.link_after(ErrorReporter);
    chain.link_after(hbse);

    let mut mount = Mount::new();
    mount
        .mount("/", chain)
        .mount("/images/", Static::new(std::path::Path::new("src/static/assets/images")));

    println!("Server running at http://localhost:3000/");
    Iron::new(mount).http("localhost:3000").unwrap();
}

#[cfg(feature = "watch")]
fn main() {
    let hbse = Arc::new(get_handlebars());
    hbse.watch(TEMPLATES_PATH);
    start(hbse)
}

#[cfg(not(feature = "watch"))]
fn main() {
    println!("Watch only enabled via --features watch option.");
    let hbse = Arc::new(get_handlebars());
    start(hbse)
}
