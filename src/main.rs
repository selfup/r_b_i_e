extern crate rust_bucket;

mod schema;

use schema::*;
use rust_bucket::*;

fn main() {
    println!("\n\n{}\n\n", create_and_read_table("first_user"));

    let mut new_user = User::new();
    new_user.name = "new name".to_string();
    new_user.age = 42;
  
    update_table("first_user", &new_user).unwrap();
    println!("{}\n\n", create_and_read_table("first_user"));

    for i in 1..101 {
       let mut looped_user = User::new();
       looped_user.name = format!("my_number_is{}", i);
       append_records("first_user", looped_user).unwrap();
    }
    
    println!("{}\n\n", create_and_read_table("first_user"));

    let mut new_user = User::new();
    new_user.name = "final name".to_string();
    new_user.age = 7;

    update_table("first_user", &new_user).unwrap();
    println!("{}\n\n", create_and_read_table("first_user"));

    drop_table("first_user").unwrap();
}

fn create_and_read_table(table: &str) -> String {
    let user = User::new();
    create_table(table, &user).unwrap();
    read_table(table).unwrap()
}