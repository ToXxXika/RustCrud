#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate dotenv;

use dotenv::dotenv;
use std::env ;
use diesel::{MysqlConnection, Connection};


mod models;
mod Schema;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = MysqlConnection::establish(&database_url).unwrap();
    let u = models::Newuser{
        id:i32::from(5485),
        name:String::from("Oussema"),
        cin:String::from("07220650")
    };
    if models::User::createUser(u,&conn){
        println!("Success");
    }else {
        println!("failed");
    }

    rocket::ignite().mount("/", routes![index]).launch();
}

