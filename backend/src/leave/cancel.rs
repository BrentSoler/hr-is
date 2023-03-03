use actix_web::{
    patch,
    web::{Data, Json},
    HttpResponse,
};

use crate::config::{errors::ErrMsg, Database};

use super::{CancelLeave, LeaveACM};

#[patch("/leave")]
pub async fn cancel_leave(db: Data<Database>, cancel_leave: Json<CancelLeave>) -> HttpResponse {
    let cancel = LeaveACM::cancel_leave(
        CancelLeave {
            Leave_Id: cancel_leave.Leave_Id.to_string(),
            Emp_Id: cancel_leave.Emp_Id.to_string(),
            Status: cancel_leave.Status,
        },
        &db.0,
    )
    .await;

    match cancel {
        Ok(()) => HttpResponse::Ok().json("Success"),
        Err(e) => HttpResponse::BadRequest().json(ErrMsg {
            err_msg: e.to_string(),
        }),
    }
}
