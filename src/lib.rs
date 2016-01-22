extern crate csv;
extern crate chrono;
extern crate rustc_serialize;

mod agency;
mod stop;
mod route;
mod calendar;
mod calendar_date;
mod stop_time;
mod trip;
mod fare_attribute;
mod fare_rule;

use agency::Agency;

#[test]
fn agencies_from_file() {
    let mut agency_reader = agency::from_file("gtfs_sample/agency.txt").unwrap();
    for agency in agency_reader.decode() {
        let agency : Agency = agency.unwrap();
        assert!(agency.agency_id.unwrap() == "FunBus");
    }
}
