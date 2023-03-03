use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};

use crate::config::{errors::ErrMsg, Database, JwtKey};

use super::EmployeeACM;

#[get("/{token}")]
pub async fn get_info(token: Path<String>, db: Data<Database>, key: Data<JwtKey>) -> HttpResponse {
    let user_info =
        EmployeeACM::get_user_and_access(&db.0, token.to_string(), key.0.to_string()).await;

    match user_info {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::BadRequest().json(ErrMsg {
            err_msg: err.to_string(),
        }),
    }
}
