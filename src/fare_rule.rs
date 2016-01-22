#[derive(RustcEncodable, RustcDecodable)] 
struct FareRule {
    fare_id : String,
    route_id : Option<String>,
    origin_id : Option<String>,
    destination_id : Option<String>,
    contains_id : Option<String>,
}
