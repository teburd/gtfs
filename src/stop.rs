#[derive(RustcEncodable, RustcDecodable)] 
struct Stop {
    stop_id : String,
    stop_code : Option<String>,
    stop_name : String,
    stop_desc : Option<String>,
    stop_lat : f64,
    stop_lon : f64,
    zone_id : Option<String>,
    stop_url : Option<String>,
    location_type : Option<String>,
    parent_station : Option<String>,
    stop_timezone : Option<String>,
    wheelchair_boarding : Option<String>,
}
