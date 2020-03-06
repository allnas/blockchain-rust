extern crate crypto;
extern crate chrono;

pub mod entity;
pub mod socket;
pub mod util;

use crate::entity::block::Block;
use crate::entity::transaction::Transaction;
use crate::util::sqlutils::connect;
use crate::util::sqlutils::init_table;
use crate::util::sqlutils::save_block;
use crate::util::sqlutils::search;
use crate::util::sqlutils::max_id;

use crypto::sha3::Sha3;
use crypto::digest::Digest;

use rustc_serialize::json;
use std::net::{UdpSocket, Ipv4Addr};

use std::io;
use sqlite::{Connection, State};
use std::borrow::BorrowMut;

fn first_block() -> String {
    let mut transactions = Vec::new();
    transactions.push(Transaction { id: 1, info: "这是创世区块".to_string() });
    transactions.push(Transaction { id: 2, info: "区块链高度为：1".to_string() });

    let hash = calculate_hash("".to_string(), transactions.as_mut(), 1);

    let block = Block {
        index: 1,
        hash,
        previous_hash: "".to_string(),
        timestamp: chrono::prelude::Local::now().timestamp_millis(),
        nonce: 1,
        transactions,
    };

    return json::encode(&block).unwrap();
}

fn block(previous_hash: String, conn: &mut Connection) -> String {
    let mut transactions = Vec::new();
    transactions.push(Transaction { id: 1, info: "共识机制生成的区块".to_string() });
    transactions.push(Transaction { id: 2, info: "区块链高度为2".to_string() });

    let mut nonce = 1;
    let mut hash;
    loop {
        hash = calculate_hash(previous_hash.to_string(), transactions.as_mut(), nonce);
        // println!("{}", hash);
        if hash.starts_with("00") {
            // println!("计算了{}次,计算出hash值为:{}", nonce, hash);
            break;
        }
        nonce += 1;
    }

    let block = Block {
        index: max_id(conn.borrow_mut()),
        hash,
        previous_hash: previous_hash.to_string(),
        timestamp: chrono::prelude::Local::now().timestamp_millis(),
        nonce,
        transactions,
    };
    return json::encode(&block).unwrap();
}

fn calculate_hash(previous_hash: String, transactions: &mut Vec<Transaction>, nonce: i32) -> String {
    let json = json::encode(transactions).unwrap();

    let mut info = "".to_string();
    info += previous_hash.as_ref();
    info += json.as_ref();
    let n = nonce.to_string();
    info += n.as_ref();

    let mut hasher = Sha3::sha3_256();

    hasher.reset();
    hasher.input_str(&info);
    let hash = hasher.result_str();
    return hash;
}

fn next_block(hash_json: String, conn: &mut Connection) -> String {
    let block_json: Block = json::decode(hash_json.as_ref()).unwrap();
    let previous_hash = block_json.hash;
    let hash_json = block(previous_hash, conn);

    return hash_json;
}

fn main() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let mcast_group: Ipv4Addr = "233.0.0.1".parse().unwrap();
    let port: u16 = 6000;
    let any = "0.0.0.0".parse().unwrap();
    let mut buffer = [0u8; 1600];
    println!("You guessed: {}",guess);
    if guess.len()> 2 {
        // client case
        println!("startclient");
        let socket = UdpSocket::bind((any, port)).expect("Could not bind client socket");
        socket.join_multicast_v4(&mcast_group, &any)
            .expect("Could not join multicast group");
        socket.recv_from(&mut buffer).expect("Failed to write to server");
    } else {
        // server case
        println!("startserver");
        let socket = UdpSocket::bind((any, 0))
            .expect("Could not write buffer as string");
        socket.send_to("Hello, world!".as_bytes(), &(mcast_group, port)).expect("Failed to write data");
    }
    // let pool = ThreadPool::new(2);
    // pool.execute(|| two());

    // pool.execute(|| one());
  
//     let mut conn = connect();
//     init_table(conn.borrow_mut());

//     let hash_json = first_block();
//     save_block(conn.borrow_mut(), 1, &hash_json);

//     let hash_json = next_block(hash_json);
//     save_block(conn.borrow_mut(), 2, &hash_json);

//     let hash_json = next_block(hash_json);
//     save_block(conn.borrow_mut(), 3, &hash_json);

//     search(conn.borrow_mut(),2);
}