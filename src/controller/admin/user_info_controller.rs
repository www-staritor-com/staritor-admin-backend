use crate::entity::Response;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SignInReq<'r> {
    code: &'r str,
    password: &'r str,
    name: &'r str,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SignInResp<'r> {
    code: &'r str,
    password: &'r str,
    name: &'r str,
}

#[allow(unused_variables)]
#[post("/sign-in", data = "<req>")]
pub fn sign_in(req: Json<SignInReq<'_>>) -> Json<Response<SignInResp>> {
    let req = req.into_inner();
    
    let resp = SignInResp {
        code: req.code,
        password: req.password,
        name: req.name,
    };
    
    Json(Response {
        code: 200,
        msg: "ok".to_string(),
        data: Some(resp),
    })
}
