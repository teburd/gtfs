#[derive(RustcEncodable, RustcDecodable)] 
struct FareAttribute {
    fare_id : String,
    price : String,
    currency_type : String,
    payment_method : u64,
    transfers : u64,
    transfer_duration : Option<u64>,
}
