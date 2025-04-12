use rocket::fairing::AdHoc;

mod resource_controller;
mod user_info_controller;

pub fn stage(prefix: String) -> AdHoc {
    AdHoc::on_ignite("Admin Init", move |rocket| async move {
        rocket.mount(
            format!("{}/{}", prefix.clone(), "admin"),
            routes![
                user_info_controller::get,
                user_info_controller::page,
                user_info_controller::sign_in,
                user_info_controller::delete,
                resource_controller::save_or_update,
                resource_controller::page,
                resource_controller::delete,
            ],
        )
    })
}
