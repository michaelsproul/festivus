use iron::prelude::*;
use iron::status::Status;

use plugin;
use typemap::Key;

use urlencoded::{UrlEncodedQuery, UrlEncodedBody, QueryMap};

use rustc_serialize::{json, Encodable};

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

/// Create a JSON or JSON-P response (if `callback` is present in the query string).
pub fn jsonp_string<T: Encodable>(req: &mut Request, val: T) -> String {
    let json_val = json::as_json(&val);
    match get_query_param(req, "callback") {
        // JSON-P response.
        Ok(callback) => {
            format!("{}({})", callback, json_val)
        }
        // Regular JSON response.
        Err(_) => {
            format!("{}", json_val)
        }
    }
}

pub fn get_query_param<'a, 'b>(req: &mut Request<'a, 'b>, param: &str) -> Result<String, ()> {
    get_param::<UrlEncodedQuery>(req, param)
}

pub fn get_body_param<'a, 'b>(req: &mut Request<'a, 'b>, param: &str) -> Result<String, ()> {
    get_param::<UrlEncodedBody>(req, param)
}

pub fn get_param<'a, 'b, T>(req: &mut Request<'a, 'b>, param: &str) -> Result<String, ()> where
    T: plugin::Plugin<Request<'a, 'b>>,
    T: Key<Value=QueryMap> {
    let req_params = match req.get_ref::<T>() {
        Ok(params) => params,
        Err(_) => return Err(())
    };
    req_params.get(param).and_then(|v| v.first().cloned()).ok_or(())
}

pub fn parse_i32(v: String) -> Result<i32, ()> {
    v.parse().map_err(|_| ())
}

pub fn parse_date(v: String) -> Result<Date, ()> {
    v.parse().map_err(|_| ())
}
