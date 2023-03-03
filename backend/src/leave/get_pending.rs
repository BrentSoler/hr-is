use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

use crate::config::{errors::ErrMsg, Database};

use super::{GetAvailableLeaves, LeaveACM};

#[post("/leave/pending")]
pub async fn get_pending(db: Data<Database>, leave_form: Json<GetAvailableLeaves>) -> HttpResponse {
    let pending = LeaveACM::get_available_leaves(
        GetAvailableLeaves {
            Leave_Type: leave_form.Leave_Type.to_string(),
            Emp_Id: leave_form.Emp_Id.to_string(),
        },
        &db.0,
    )
    .await;

    match pending {
        Ok(pending) => HttpResponse::Ok().json(pending),
        Err(e) => HttpResponse::BadRequest().json(ErrMsg {
            err_msg: e.to_string(),
        }),
    }
}
