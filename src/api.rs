use iron;
use iron::status::Status::{self, BadRequest};
use iron::prelude::*;
use router::Router;
use urlencoded::UrlEncodedBody;

use iron_pg::{PostgresMiddleware, PostgresReqExt};

pub fn create_router() -> Router {
    router! {
        get "/" => root_handler,
        get "power" => get_power,
        post "power" => post_power,
        get "energy" => get_energy
    }
}

fn get_power(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with("GET /power"))
}

// Parse a POST request with body of the form: peak=X&offpeak=Y.
fn post_power(req: &mut Request) -> IronResult<Response> {
    let req_body = match req.get_ref::<UrlEncodedBody>() {
        Ok(body) => body,
        Err(e) => return err_response(BadRequest, "Not URL-encoded.")
    };
    
    let (peak_str, offpeak_str) = match (req_body.get("peak").and_then(|v| v.first()),
                                         req_body.get("offpeak").and_then(|v| v.first())) {
        (Some(peak), Some(offpeak)) => (peak, offpeak),
        _ => return err_response(BadRequest, "Values for 'peak' and 'offpeak' not given.")
    };
    
    let (peak, offpeak) : (u32, u32) = match (peak_str.parse(), offpeak_str.parse()) {
        (Ok(x), Ok(y)) => (x, y),
        _ => return err_response(BadRequest, "Non-integer power values.")
    };
    println!("Received peak={} offpeak={}", peak, offpeak);
    
    Ok(Response::with((Status::Ok, "Yay that worked!")))
}

fn get_energy(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with("GET /energy"))
}

fn err_response(status: Status, msg: &str) -> IronResult<Response> {
    Ok(Response::with((status, msg)))
}

fn root_handler(req: &mut Request) -> IronResult<Response> {
    let conn = req.db_conn();
    let stmt = conn.prepare("SELECT * FROM power").unwrap();
    let rows = stmt.query(&[]).unwrap();
    let response_str = format!("{:?}", rows);
    Ok(Response::with((iron::status::Ok, response_str)))
}
