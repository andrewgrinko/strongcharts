#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::io::{Read, BufReader};
use std::io::prelude::*;
use::rocket::Data;
use rocket_contrib::json::JsonValue;
use strongcharts;

const LIMIT: u64 = 1024 * 1024; // 1MB

#[post("/upload", format = "multipart/form-data", data = "<data>")]
fn upload(data: Data) -> JsonValue {
    let data = data.open().take(LIMIT);
    let parsed = get_file(data);
    let res = strongcharts::read_csv(&parsed);
    match res {
        Err(e) => {
            print!("err: {}, data: {}", e, parsed);
            return json!({ "status": "error" })
        },
        Ok(result) => return json!(result)
    }
}

fn get_file<T: Read>(f: T) -> String {
    let mut result = String::new();
    let r = BufReader::new(f);
    for (index, line) in r.lines().enumerate() {
        let line = line.unwrap();
        if index > 3 && !line.starts_with("---------") {
            result.push_str(&format!("{}\n", &line));
        }
    }
    result
}

fn main() {
    rocket::ignite().mount("/", routes![upload]).launch();
}
