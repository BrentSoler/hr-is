use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

use crate::config::{errors::ErrMsg, Database, JwtKey};

use super::{EmployeeACM, EmployeeLogin};

#[post("/login")]
pub async fn emp_login(
    db: Data<Database>,
    key: Data<JwtKey>,
    emp_form: Json<EmployeeLogin>,
) -> HttpResponse {
    let login = EmployeeACM::login(
        &db.0,
        EmployeeLogin {
            Emp_Id: emp_form.Emp_Id.to_string(),
            Emp_Pswd: emp_form.Emp_Pswd.to_string(),
        },
        key.0.clone(),
    )
    .await;

    match login {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => HttpResponse::BadRequest().json(ErrMsg {
            err_msg: err.to_string(),
        }),
    }
}
