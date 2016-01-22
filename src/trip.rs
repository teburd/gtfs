#[derive(RustcEncodable, RustcDecodable)] 
struct Trip {
    route_id : String,
    service_id : String,
    trip_id : String,
    trip_headsign : Option<String>,
    trip_short_name : Option<String>,
    direction_id : Option<u64>,
    block_id : Option<String>,
    shape_id : Option<String>,
    wheelchair_accessible : Option<String>,
    bikes_allowed : Option<String>,
}

