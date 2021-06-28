extern crate postgres;

use postgres::*;
use postgres::Connection;
use std::fs::File;
use std::fs;

#[derive(Debug)]
struct Info {
    id: i32,
    height: String,
    r#type: String,
    txhash: String,
    key: String,
    amount: String,
    owner: String,
    gas_used: String,
    fee: String,
}

fn find_something() {
    let file = fs::File::open("src/unbond2.log")
        .expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");
    let fee = json.get("fee")
        .expect("file should have fee key");
}

fn main() {
    let conn = Connection::connect("postgresql://postgres:postgres@localhost:5442", TlsMode::None)
        .unwrap();

    for row in &conn.query("SELECT height, r#type,\
     txhash,r#key,amount,owner,gas_used,fee FROM wallet WHERE r#type='wasm' & key='mixbond'", &[]).unwrap() {
        let wallet = Info {
            id: row.get(0),
            height: row.get(1),
            r#type: row.get(3),
            txhash: row.get(4),
            r#key: row.get(5),
            amount: row.get(6),
            owner: row.get(7),
            gas_used: row.get(8),
            fee: row.get(9),
        };
        println!("{:#?}", wallet);
    }
}