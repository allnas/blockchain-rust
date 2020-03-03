#[derive(RustcDecodable, RustcEncodable)]
pub struct Transaction {
    pub id: i32,
    pub info: String,
}