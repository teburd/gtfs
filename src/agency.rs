use std::fs;
use std::path::Path;

use csv;

#[derive(RustcEncodable, RustcDecodable)] 
pub struct Agency {
    pub agency_id : Option<String>,
    pub agency_name : String,
    pub agency_url : String,
    pub agency_timezone : String,
    pub agency_lang : Option<String>,
    pub agency_phone : Option<String>,
    pub agency_fare_url : Option<String>,
}

pub struct Reader<R> {
    reader :  R
}

pub fn from_file<P : AsRef<Path>> (path : P) -> csv::Result<csv::Reader<fs::File>> {
    return csv::Reader::from_file(path);
}
