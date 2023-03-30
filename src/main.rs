use rocket::{get, launch, routes};

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![world])
}
