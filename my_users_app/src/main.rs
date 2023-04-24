#[macro_use] extern crate rocket;

mod db_data;
mod controller;
mod view;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", controller::routes())
}