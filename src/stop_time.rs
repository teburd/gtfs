#[derive(RustcEncodable, RustcDecodable)] 
struct StopTime {
    trip_id : String,
    arrival_time : String,
    departure_time : String,
    stop_id : String,
    stop_sequence : u64,
    stop_headsign : Option<String>,
    pickup_type : Option<String>,
    drop_off_type : Option<String>,
    shape_dist_traveled : Option<String>,
    timepoint : Option<String>,
}
