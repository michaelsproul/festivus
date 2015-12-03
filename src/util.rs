use iron::prelude::*;
use iron::status::Status;

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
