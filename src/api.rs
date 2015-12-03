use std::collections::HashSet;
use std::collections::BTreeMap;

use iron;
use iron::status::Status::{self, BadRequest, InternalServerError};
use iron::prelude::*;
use urlencoded::UrlEncodedQuery;

use router::Router;

use iron_pg::PostgresReqExt;

use types::*;
use util::*;
use db;
use compute::integral;

const INSERT_SQL: &'static str = "INSERT INTO power (time, total, hot_water, solar) VALUES (now(), $1, $2, $3)";

pub fn create_router() -> Router {
    router! {
        get "/" => root_handler,
        get "power" => get_power,
        post "power" => post_power,
        get "energy" => get_energy
    }
}

// Parse start and end params from a query string.
fn get_start_and_end(req: &mut Request) -> WebResult<(Date, Date)> {
    match (get_query_param(req, "start").and_then(parse_date),
           get_query_param(req, "end").and_then(parse_date)) {
        (Ok(x), Ok(y)) => Ok((x, y)),
        _ => Err(err_response(BadRequest, "Start and end dates not specified."))
    }
}

// Parse stream values from a query string.
fn get_streams(req: &mut Request) -> Option<HashSet<PowerStream>> {
    req.get_ref::<UrlEncodedQuery>().ok()
       .and_then(|query_map| query_map.get("stream"))
       .and_then(|stream_strings| stream_strings.into_iter()
                                                .map(|s| PowerStream::from_str(&s[..]))
                                                .collect())
}

// Check if the query string contains the `energy` query.
fn is_energy_query(req: &mut Request) -> bool {
    req.get_ref::<UrlEncodedQuery>()
       .map(|query_map| query_map.contains_key("energy"))
       .unwrap_or(false)
}

fn all_streams() -> HashSet<PowerStream> {
    let mut streams = HashSet::new();
    streams.extend(&[Total, HotWater, Solar]);
    streams
}

// GET /power?start=X&end=X
fn get_power(req: &mut Request) -> IronResult<Response> {
    let (start, end) = try_res!(get_start_and_end(req));
    let streams = get_streams(req).unwrap_or_else(|| all_streams());
    let compute_energy = is_energy_query(req);
    println!("start: {:?}, end: {:?}, streams: {:?}", start, end, streams);

    let power_data: Vec<Power> = try_res!(db::get_power(req, start, end));

    let mut result = BTreeMap::new();
    for stream in streams {
        result.insert(
            stream.as_str(),
            StreamJson::from_power_data(&power_data, stream, compute_energy)
        );
    }

    let data_string = jsonp_string(req, result);

    Ok(Response::with((Status::Ok, data_string)))
}

// Parse a POST request with body of the form: total=X&hot_water=Y&solar=Z.
fn post_power(req: &mut Request) -> IronResult<Response> {
    let (total, hot_water, solar) =
        match (get_body_param(req, "total").and_then(parse_i32),
               get_body_param(req, "hot_water").and_then(parse_i32),
               get_body_param(req, "solar").and_then(parse_i32)) {
        (Ok(x), Ok(y), Ok(z)) if x >= 0 && y >= 0 && z >= 0 => (x, y, z),
        _ => return Ok(err_response(BadRequest, "Unable to parse integer values for total, hot_water and solar."))
    };
    println!("Received total={}, hot_water={}, solar={}", total, hot_water, solar);

    // Insert into DB.
    let conn = req.db_conn();
    match conn.prepare(INSERT_SQL).and_then(|s| s.execute(&[&total, &hot_water, &solar])) {
        // 1 row modified, good!
        Ok(1) => (),
        x => {
            println!("ERROR - DB insert response:\n{:?}", x);
            return Ok(err_response(InternalServerError, "Error inserting values into DB."));
        }
    }

    Ok(Response::with((Status::Ok, "Success.")))
}

// GET /energy?start=X&end=Y&stream=(total|hot_water|solar)
fn get_energy(req: &mut Request) -> IronResult<Response> {
    let (start, end) = try_res!(get_start_and_end(req));
    let stream = match get_query_param(req, "stream") {
        Ok(ref s) if s == "total" => Total,
        Ok(ref s) if s == "hot_water" => HotWater,
        Ok(ref s) if s == "solar" => Solar,
        _ => return Ok(err_response(BadRequest, "Missing or invalid value for stream parameter."))
    };

    let power = try_res!(db::get_power(req, start, end));
    let energy = integral(&power, stream);
    let response_str = format!("{}", energy);

    Ok(Response::with((Status::Ok, response_str)))
}

fn root_handler(req: &mut Request) -> IronResult<Response> {
    let conn = req.db_conn();
    let stmt = conn.prepare("SELECT * FROM power").unwrap();
    let rows = stmt.query(&[]).unwrap();
    let response_str = format!("{:?}", rows);
    Ok(Response::with((iron::status::Ok, response_str)))
}
