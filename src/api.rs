use iron;
use iron::status::Status::{self, BadRequest, InternalServerError};
use iron::prelude::*;
use router::Router;
use urlencoded::UrlEncodedBody;
use iron_pg::PostgresReqExt;
use serde_json;

use types::{Power, PowerView};

const INSERT_SQL: &'static str = "INSERT INTO power (time, peak, offpeak) VALUES (now(), $1, $2)";
const QUERY_SQL: &'static str = "SELECT time, peak, offpeak FROM power";

pub fn create_router() -> Router {
    router! {
        get "/" => root_handler,
        get "power" => get_power,
        post "power" => post_power,
        get "energy" => get_energy
    }
}

// GET /power?start=X&end=X
fn get_power(req: &mut Request) -> IronResult<Response> {
    let conn = req.db_conn();
    let stmt = conn.prepare(QUERY_SQL).unwrap();
    let query = stmt.query(&[]);
    let rows = match query {
        Ok(rows) => rows,
        Err(e) => {
            println!("ERROR - DB query response:\n{:?}", e);
            return err_response(InternalServerError, "Error querying DB");
        }
    };

    let data: Vec<PowerView> = rows.into_iter().map(|row| {
        Power {
            time: row.get(0),
            peak: row.get(1),
            offpeak: row.get(2)
        }.view()
    }).collect();

    // FIXME: could use some sort of insane iterator streaming instead.
    let data_string = serde_json::ser::to_string(&data).unwrap();

    Ok(Response::with((Status::Ok, data_string)))
}

// Parse a POST request with body of the form: peak=X&offpeak=Y.
fn post_power(req: &mut Request) -> IronResult<Response> {
    let req_body = match req.get::<UrlEncodedBody>() {
        Ok(body) => body,
        Err(_) => return err_response(BadRequest, "Not URL-encoded.")
    };

    let (peak_str, offpeak_str) = match (req_body.get("peak").and_then(|v| v.first()),
                                         req_body.get("offpeak").and_then(|v| v.first())) {
        (Some(peak), Some(offpeak)) => (peak, offpeak),
        _ => return err_response(BadRequest, "Values for 'peak' and 'offpeak' not given.")
    };

    let (peak, offpeak) : (i32, i32) = match (peak_str.parse(), offpeak_str.parse()) {
        (Ok(x), Ok(y)) if x >= 0 && y >= 0 => (x, y),
        _ => return err_response(BadRequest, "Non-integer power values.")
    };
    println!("Received peak={} offpeak={}", peak, offpeak);

    // Insert into DB.
    let conn = req.db_conn();
    match conn.prepare(INSERT_SQL).and_then(|s| s.execute(&[&peak, &offpeak])) {
        // 1 row modified, good!
        Ok(1) => (),
        x => {
            println!("ERROR - DB insert response:\n{:?}", x);
            return err_response(InternalServerError, "Error inserting values into DB.");
        }
    }

    Ok(Response::with((Status::Ok, "Success.")))
}

fn get_energy(_req: &mut Request) -> IronResult<Response> {
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
