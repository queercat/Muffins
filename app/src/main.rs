use rocket::serde::{json::Json};

#[macro_use] extern crate rocket;
use muffins::muffin;

#[get("/")]
fn serve() -> Json<muffin::Muffin> {
    Json(muffin::bake_muffin())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![serve])
}
