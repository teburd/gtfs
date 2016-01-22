#[derive(RustcEncodable, RustcDecodable)] 
struct Route {
    route_id : String,
    agency_id : Option<String>,
    route_short_name : String,
    route_long_name : String,
    route_desc : Option<String>,
    route_type : u64,
    route_url : Option<String>,
    route_color : Option<String>,
    route_text_color : Option<String>,
}

