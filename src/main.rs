extern crate rust_bucket;

mod schema;

use schema::*;
use rust_bucket::*;

extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, create_and_read_table("user_one"))))
    }).http("localhost:3000").unwrap();
}

fn create_and_read_table(table: &str) -> String {
    let user = User::new();

    create_table("user_one", &user).unwrap();

    read_table(table).unwrap()
}