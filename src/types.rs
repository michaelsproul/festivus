use iron::prelude::*;
use chrono::{DateTime, Local};

pub use self::PowerStream::*;

/// Dates are all expressed in the current local timezone.
/// Input dates should be ISO8601 with explicit offsets, or UTC.
/// All output dates are local.
pub type Date = DateTime<Local>;

pub struct Power {
    pub time: Date,
    pub total: i32,
    pub hot_water: i32,
    pub solar: i32
}

pub enum PowerStream {
    Total,
    HotWater,
    Solar
}

#[derive(RustcEncodable)]
pub struct PlotJson<T> {
    /// Date-time strings.
    pub x: Vec<String>,
    // Values.
    pub y: Vec<T>
}

pub type WebResult<T> = Result<T, Response>;

/*
use std::iter::Iterator;
use iron::response::{ResponseBody, WriteBody};
use serde_json;

enum ValueIter(I) where I is an iterator.

impl WriteBody for I where I: Iterator<Item=PowerView> {
    fn write_body(&mut self, res: &mut ResponseBody) -> Result<(), ()> {
        serde_json::ser::to_writer(res, self)
    }
}
*/
