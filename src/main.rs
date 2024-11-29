#[macro_use] extern crate rocket;

// Define a simple route
#[get("/")]
fn index() -> &'static str {
    "Welcome to the blogging app!"
}

// Launch the Rocket server
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
