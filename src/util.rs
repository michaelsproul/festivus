use iron::prelude::*;
use iron::status::Status;
use plugin;
use typemap::Key;
use urlencoded::{UrlEncodedQuery, UrlEncodedBody, QueryMap};

use types::*;

macro_rules! try_res {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(response) => return Ok(response)
        }
    }
}

pub fn err_response(status: Status, msg: &str) -> Response {
    Response::with((status, msg))
}

/// Create a Plotly timestamp of the form "YYYY-MM-DD HH:mm:ss", in the local timezone.
pub fn timestamp(d: &Date) -> String {
    format!("{}", d.format("%Y-%m-%d %H:%M:%S"))
}

/// Send a JSON or JSON-P response (if `callback` is present in the query string).
//pub fn json_response()

pub fn get_query_param<'a, 'b>(req: &mut Request<'a, 'b>, param: &str) -> Result<String, ()> {
    get_param::<UrlEncodedQuery>(req, param)
}

pub fn get_body_param<'a, 'b>(req: &mut Request<'a, 'b>, param: &str) -> Result<String, ()> {
    get_param::<UrlEncodedBody>(req, param)
}

pub fn get_param<'a, 'b, T>(req: &mut Request<'a, 'b>, param: &str) -> Result<String, ()> where
    T: plugin::Plugin<Request<'a, 'b>>,
    T: Key<Value=QueryMap> {
    let req_body = match req.get_ref::<T>() {
        Ok(body) => body,
        Err(_) => return Err(())
    };
    req_body.get(param).and_then(|v| v.first().cloned()).ok_or(())
}

pub fn parse_i32(v: String) -> Result<i32, ()> {
    v.parse().map_err(|_| ())
}

pub fn parse_date(v: String) -> Result<Date, ()> {
    v.parse().map_err(|_| ())
}
