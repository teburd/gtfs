#[derive(RustcEncodable, RustcDecodable)] 
struct CalendarDate {
    service_id : String,
    date : String,
    exception_type : String,
}
