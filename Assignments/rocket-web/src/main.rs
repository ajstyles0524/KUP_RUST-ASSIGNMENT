#[macro_use] extern crate rocket;
use rocket::response::content::Json;
#[get("/")]
fn hello() -> Json<&'static str> {
    Json("{
    'message': 'Hello Rust!'
  }")
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}