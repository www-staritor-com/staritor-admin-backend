use crate::entity::base::error::BizError::{CustomError, NotFound};
use crate::entity::base::response::Response;
use rocket::response::{content, Responder};
use rocket::serde::json::serde_json::json;
use rocket::{response, Request};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BizError {
    #[error("{0}")]
    CustomError(String),
    #[error("the data not found, parameter: {0}")]
    NotFound(String),
    #[error("db error")]
    DbError(#[from] sea_orm::DbErr),
    #[error("validation error")]
    ValidationError(#[from] validator::ValidationErrors),
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for BizError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        let response: Response<()> = match self {
            CustomError(str) => {
                Response::error(rocket::http::Status::BadRequest.code, str.to_owned())
            }
            NotFound(str) => Response::error(
                rocket::http::Status::NotFound.code,
                NotFound(str).to_string(),
            ),
            BizError::DbError(err) => {
                error!("{:?}", err);
                Response::error(
                    rocket::http::Status::InternalServerError.code,
                    err.to_string(),
                )
            }
            BizError::ValidationError(err) => {
                Response::error(rocket::http::Status::MethodNotAllowed.code, err.to_string())
            }
        };
        content::RawJson(json!(response)).respond_to(request)
    }
}
