
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::collections::HashMap;
use rocket::http::Status;
use rocket::response::status;

struct MyHashMap(HashMap<String, i32>);

#[get("/<key>")]
fn index(key: String) -> Result<String, status::Custom<&'static str>> { 
    let mut my_map = HashMap::new();
    my_map.insert("Agriculture".to_string(), 60);
    my_map.insert("Apparel".to_string(), 40);
    my_map.insert("Banking".to_string(), 45);
    my_map.insert("Biotechnology".to_string(), 45);
    my_map.insert("Chemicals".to_string(), 55);
    my_map.insert("Communications".to_string(), 55);
    my_map.insert("Construction".to_string(), 55);
    my_map.insert("Consulting".to_string(), 48);
    my_map.insert("Education".to_string(), 29);
    my_map.insert("Electronics".to_string(), 36);
    my_map.insert("Energy".to_string(), 56);
    my_map.insert("Engineering".to_string(), 38);
    my_map.insert("Entertainment".to_string(), 38);
    my_map.insert("Environmental".to_string(), 38);
    my_map.insert("Finance".to_string(), 48);
    my_map.insert("Fintech".to_string(), 38);
    my_map.insert("Government".to_string(), 60);
    my_map.insert("Healthcare".to_string(), 59);
    my_map.insert("Hospitality".to_string(), 56);
    my_map.insert("Insurance".to_string(), 40);
    my_map.insert("Machinery".to_string(), 60);
    my_map.insert("Manufacturing".to_string(), 75);
    my_map.insert("Media".to_string(), 47);
    my_map.insert("Jewelry".to_string(), 50);
    my_map.insert("Recreation".to_string(), 48);
    my_map.insert("Retail".to_string(), 41);
    my_map.insert("Shipping".to_string(), 52);
    my_map.insert("Technology".to_string(), 79);
    my_map.insert("Telecommunications".to_string(), 64);
    my_map.insert("Transportation".to_string(), 55);
    my_map.insert("Utilities".to_string(), 39);

    match my_map.get(&key) {
        Some(value) => Ok(value.to_string()),
        None => Err(status::Custom(Status::NotFound, "Key not found")),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
      .mount("/", routes![index])
}

