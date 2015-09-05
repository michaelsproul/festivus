use chrono::{DateTime, Local};

pub struct Power {
    pub time: DateTime<Local>,
    pub peak: i32,
    pub offpeak: i32,
}

impl Power {
    pub fn view(self) -> PowerView {
        PowerView {
            time: self.time.to_rfc3339(),
            peak: self.peak,
            offpeak: self.offpeak
        }
    }
}

#[derive(Serialize)]
pub struct PowerView {
    pub time: String,
    pub peak: i32,
    pub offpeak: i32
}

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
