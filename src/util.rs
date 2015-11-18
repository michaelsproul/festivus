use iron::prelude::*;
use iron::status::Status;

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
