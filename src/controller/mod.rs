use crate::entity::base::response::Response;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::Request;

mod admin;

#[catch(default)]
fn default_catch_all(status: Status, req: &Request<'_>) -> Response<()> {
    Response::error(status.code, req.uri().to_string())
}

pub fn stage(prefix: String) -> AdHoc {
    AdHoc::on_ignite("Controller Init", move |rocket| async move {
        rocket
            .register(
                format!("{}/{}", prefix.clone(), "api"),
                catchers![default_catch_all],
            )
            .attach(admin::stage(format!("{}/{}", prefix.clone(), "api")))
    })
}
