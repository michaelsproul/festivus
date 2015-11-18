use iron;
use iron::status::Status::{self, BadRequest, InternalServerError};
use iron::prelude::*;

use plugin;
use typemap::Key;

use router::Router;
use urlencoded::{UrlEncodedQuery, UrlEncodedBody, QueryMap};
use iron_pg::PostgresReqExt;

use serde_json;
use chrono::{DateTime, FixedOffset};

use types::{Power, PowerView};

const INSERT_SQL: &'static str = "INSERT INTO power (time, peak, offpeak) VALUES (now(), $1, $2)";
const QUERY_SQL: &'static str =
    "SELECT time, peak, offpeak FROM power WHERE time >= $1 AND time <= $2";

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
    // Parse start and end params.
    let (start, end) = match (get_query_param(req, "start").and_then(parse_date),
                              get_query_param(req, "end").and_then(parse_date)) {
        (Ok(x), Ok(y)) => (x, y),
        _ => return err_response(BadRequest, "Start and end dates not specified.")
    };
    println!("start: {:?}. end: {:?}", start, end);

    // Retrieve rows from the DB.
    let conn = req.db_conn();
    let stmt = conn.prepare(QUERY_SQL).unwrap();
    let query = stmt.query(&[&start, &end]);
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
    let (peak, offpeak) = match (get_body_param(req, "peak").and_then(parse_i32),
                                 get_body_param(req, "offpeak").and_then(parse_i32)) {
        (Ok(x), Ok(y)) if x >= 0 && y >= 0 => (x, y),
        _ => return err_response(BadRequest, "Unable to parse integer values for peak+offpeak.")
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

fn get_query_param<'a, 'b>(req: &mut Request<'a, 'b>, param: &str) -> Result<String, ()> {
    get_param::<UrlEncodedQuery>(req, param)
}

fn get_body_param<'a, 'b>(req: &mut Request<'a, 'b>, param: &str) -> Result<String, ()> {
    get_param::<UrlEncodedBody>(req, param)
}

fn get_param<'a, 'b, T>(req: &mut Request<'a, 'b>, param: &str) -> Result<String, ()> where
    T: plugin::Plugin<Request<'a, 'b>>,
    T: Key<Value=QueryMap> {
    let req_body = match req.get_ref::<T>() {
        Ok(body) => body,
        Err(_) => return Err(())
    };
    req_body.get(param).and_then(|v| v.first().cloned()).ok_or(())
}

fn parse_i32(v: String) -> Result<i32, ()> {
    v.parse().map_err(|_| ())
}

fn parse_date(v: String) -> Result<DateTime<FixedOffset>, ()> {
    v.parse().map_err(|_| ())
}
