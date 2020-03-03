use crate::entity::transaction::Transaction;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Block {
    pub index: i32,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub nonce: i32,
    pub transactions: Vec<Transaction>
}