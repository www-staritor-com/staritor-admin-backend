use rocket::fairing::AdHoc;

mod admin;

pub fn stage(prefix: String) -> AdHoc {
    AdHoc::on_ignite("Controller Init", move |rocket| async move {
        rocket.attach(admin::stage(format!("{}/{}", prefix.clone(), "api")))
    })
}
