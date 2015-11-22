//! Database interop code.

use iron::prelude::*;
use iron::status::Status::InternalServerError;
use iron_pg::PostgresReqExt;

use types::*;
use util::err_response;

const QUERY_SQL: &'static str =
    "SELECT time, ch1, ch2, ch3 FROM power WHERE time >= $1 AND time <= $2";

// Retrieve rows from the DB.
pub fn get_power(req: &mut Request, start: Date, end: Date) -> WebResult<Vec<Power>> {
    let conn = req.db_conn();
    let stmt = conn.prepare(QUERY_SQL).unwrap();
    let query = stmt.query(&[&start, &end]);
    query.map(|rows| {
        rows.into_iter().map(|row| {
            Power {
                time: row.get(0),
                peak: row.get(1),
                offpeak: row.get(2)
            }
        }).collect()
    }).map_err(|_| err_response(InternalServerError, "Error querying DB"))
}
