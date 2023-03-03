use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};

use super::LeaveACM;
use crate::config::{errors::ErrMsg, Database};

#[get("/leave/credits/{id}")]
pub async fn get_credits(emp_id: Path<String>, db: Data<Database>) -> HttpResponse {
    let leave = LeaveACM::get_user_leave_credits(
        &db.0,
        crate::users::EmpID {
            Emp_Id: emp_id.to_string(),
        },
    )
    .await;

    return match leave {
        Ok(avail) => HttpResponse::Ok().json(avail),
        Err(err) => HttpResponse::BadRequest().json(ErrMsg {
            err_msg: err.to_string(),
        }),
    };
}
