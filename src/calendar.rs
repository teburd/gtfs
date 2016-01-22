#[derive(RustcEncodable, RustcDecodable)] 
struct Calendar {
    service_id : String,
    monday : bool,
    tuesday : bool,
    wednesday : bool,
    thursday : bool, 
    friday : bool,
    saturday : bool,
    sunday : bool,
    start_date : String,
    end_date : String,
}
