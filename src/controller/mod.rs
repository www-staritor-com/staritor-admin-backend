mod admin;

pub fn routers() -> Vec<(String, Vec<rocket::Route>)> {
    let mut routers = Vec::new();

    for (path, router) in admin::routers() {
        routers.push((format!("/{}{}", "admin", path), router));
    }

    routers
}
