extern crate router;

use iron::prelude::*;
use iron::status;
use routes::router::Router;
use handlebars::Template;

fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("page/index", "".to_string())).set_mut(status::Ok);
    Ok(resp)
}

fn sign_up(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("page/sign_up", "".to_string())).set_mut(status::Ok);
    Ok(resp)
}

fn sign_in(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("page/sign_in", "".to_string())).set_mut(status::Ok);
    Ok(resp)
}

fn sign_out(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("page/sign_out", "".to_string())).set_mut(status::Ok);
    Ok(resp)
}

pub fn new() -> Router {
    let mut router = Router::new();
    router
        .get("/", index)
        .get("/sign-up", sign_up)
        .get("/sign-in", sign_in)
        .get("/sign-out", sign_out);
    router
}
