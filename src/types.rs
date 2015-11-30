use iron::prelude::*;
use chrono::{DateTime, FixedOffset};

pub use self::PowerStream::*;

pub type Date = DateTime<FixedOffset>;

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

impl Power {
    pub fn view(self) -> PowerView {
        PowerView {
            time: self.time.to_rfc3339(),
            total: self.total,
            hot_water: self.hot_water,
            solar: self.solar
        }
    }
}

#[derive(RustcEncodable)]
pub struct PowerView {
    pub time: String,
    pub total: i32,
    pub hot_water: i32,
    pub solar: i32
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
