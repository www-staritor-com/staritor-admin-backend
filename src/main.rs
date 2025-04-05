#[macro_use]
extern crate rocket;
mod controller;
mod entity;

#[launch]
fn rocket() -> _ {
    let mut rocket = rocket::build();

    for (path, router) in controller::routers() {
        rocket = rocket.mount(format!("/{}{}","api", path), router);
    }

    rocket
}
