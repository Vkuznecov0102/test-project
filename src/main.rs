mod database;

extern crate rustc_serialize;
extern crate postgres;
use postgres::*;

use std::fs::File;
use serde::{Serialize, Deserialize};

use std::error::Error;
use std::io::BufReader;
use std::path::Path;
use core::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
struct Info{
    height: String,
    r#type: String,
    txhash: String,
    key: String,
    amount: String,
    owner: String,
    gas_used: String
}

fn read_info_from_file<P: AsRef<Path>>(path: P) -> Result<Info, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Info`.
    let u = serde_json::from_reader(reader)?;

    // Return the `Info`.
    Ok(u)
}

fn main() {
    let u = read_info_from_file("src/unbond2.log").unwrap();
    println!("{:#?}", u);
}
