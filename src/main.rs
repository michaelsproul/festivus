extern crate iron;
#[macro_use] extern crate router;
extern crate urlencoded;
extern crate iron_postgres_middleware as iron_pg;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
extern crate chrono;

mod api;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error as ErrorT;

use iron::prelude::*;
use iron_pg::PostgresMiddleware;

type Error = Box<ErrorT>;
type Conn = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;

const DB_USER: &'static str = "postgres";
const DB_NAME: &'static str = "festivus";

const TABLES: &'static [&'static str] = &["power", "energy"];
const SCHEMA_DIR: &'static str = "schema";

fn get_schema(name: &str) -> Result<String, Error> {
    let path = Path::new(SCHEMA_DIR).join(format!("{}.sql", name));
    let mut s = String::new();
    try!(File::open(path).and_then(|mut f| f.read_to_string(&mut s)));
    Ok(s)
}

fn table_exists(conn: &Conn, table: &str) -> Result<bool, Error> {
    let check_exists = r##"
    SELECT EXISTS(SELECT 1 FROM information_schema.tables
    WHERE table_catalog = $1 AND
          table_schema = 'public' AND
          table_name = $2
    );"##;

    let stmt = try!(conn.prepare(check_exists));
    let rows = try!(stmt.query(&[&DB_NAME, &table]));
    Ok(rows.get(0).get("exists"))
}

/// Set the database up for the first time.
fn initialise_db(conn: &Conn) -> Result<(), Error> {
    for &table in TABLES {
        if try!(table_exists(conn, table)) {
            println!("Found table {}.", table);
            continue
        }
        let schema = try!(get_schema(table));
        try!(conn.execute(&schema, &[]));
        println!("Created table {}.", table);
    }
    Ok(())
}

fn main() {
    let db_manager = PostgresMiddleware::new(&format!("postgres://{}@localhost/{}", DB_USER, DB_NAME));
    
    if let Err(e) = initialise_db(&db_manager.pool.get().unwrap()) {
        println!("{:?}", e);
        panic!("DB initialisation error");
    }
    
    let router = api::create_router();

    let mut server = Chain::new(router);
    server.link_before(db_manager);

    Iron::new(server).http("localhost:3000").unwrap();
}
