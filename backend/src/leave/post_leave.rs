use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

use crate::config::{errors::ErrMsg, Database};

use super::{LeaveACM, LeaveInsert};

#[post("/leave")]
pub async fn post_leave(leave_form: Json<LeaveInsert>, db: Data<Database>) -> HttpResponse {
    let Details = &leave_form.Details;

    let insert = LeaveACM::apply_leave(
        &db.0,
        LeaveInsert {
            Emp_Id: leave_form.Emp_Id.clone(),
            Leave_Id: leave_form.Leave_Id.clone(),
            Lea_Swithoutpay: leave_form.Lea_Swithoutpay,
            Lea_Swithpay: leave_form.Lea_Swithpay,
            Date_From: leave_form.Date_From.clone(),
            Date_To: leave_form.Date_To.clone(),
            Reason: leave_form.Reason.clone(),
            Details: Details.to_vec(),
        },
    )
    .await;

    match insert {
        Ok(_) => HttpResponse::Ok().json("{msg:Success}"),
        Err(e) => HttpResponse::BadRequest().json(ErrMsg {
            err_msg: e.to_string(),
        }),
    }
}
