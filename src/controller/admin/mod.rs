mod user_info_controller;

pub fn routers() -> Vec<(String, Vec<rocket::Route>)> {
    let mut routers = Vec::new();
    routers.push(("/user".to_owned(), routes![user_info_controller::sign_in]));
    routers
}
