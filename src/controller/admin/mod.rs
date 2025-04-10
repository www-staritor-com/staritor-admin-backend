use rocket::fairing::AdHoc;

mod user_info_controller;

pub fn stage(prefix: String) -> AdHoc {
    AdHoc::on_ignite("Admin Init", move |rocket| async move {
        rocket.mount(
            format!("{}/{}", prefix.clone(), "admin"),
            routes![
                user_info_controller::sign_in,
                user_info_controller::get_user,
                user_info_controller::delete_user,
            ],
        )
    })
}
