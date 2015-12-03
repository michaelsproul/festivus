use iron::prelude::*;
use chrono::{DateTime, Local};

use util::*;
use compute::*;

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

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum PowerStream {
    Total,
    HotWater,
    Solar
}

impl PowerStream {
    pub fn from_str(s: &str) -> Option<PowerStream> {
        match s {
            s if s == "total" => Some(Total),
            s if s == "hot_water" => Some(HotWater),
            s if s == "solar" => Some(Solar),
            _ => None
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            Total => "total",
            HotWater => "hot_water",
            Solar => "solar"
        }
    }
}

#[derive(RustcEncodable, PartialEq)]
pub struct StreamJson {
    /// Date-time strings.
    pub x: Vec<String>,
    /// Values.
    pub y: Vec<i32>,
    /// Energy integral for the given data.
    pub energy: Option<i64>
}

impl StreamJson {
    pub fn from_power_data(power_data: &[Power], stream: PowerStream, compute_energy: bool)
    -> StreamJson {
        let x_values: Vec<String> = power_data.iter().map(|x| &x.time).map(timestamp).collect();
        let y_values: Vec<i32> = power_data.iter().map(|x| x.total).collect();
        let energy = if compute_energy {
            Some(integral(power_data, stream))
        } else {
            None
        };
        StreamJson { x: x_values, y: y_values, energy: energy }
    }
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
