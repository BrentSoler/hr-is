use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};

use crate::config::{errors::ErrMsg, Database};

use super::COAACM;

#[get("/coa/{emp_id}")]
pub async fn get_coa(id: Path<String>, db: Data<Database>) -> HttpResponse {
    let coa = COAACM::get_user_coa(
        &db.0,
        crate::users::EmpID {
            Emp_Id: id.to_string(),
        },
    )
    .await;

    match coa {
        Ok(coas) => HttpResponse::Ok().json(coas),
        Err(e) => HttpResponse::BadRequest().json(ErrMsg {
            err_msg: e.to_string(),
        }),
    }
}
