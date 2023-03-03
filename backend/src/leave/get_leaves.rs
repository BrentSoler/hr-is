use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};

use crate::config::{errors::ErrMsg, Database};

use super::LeaveACM;

#[get("/leave/{emp_id}")]
pub async fn get_leaves(id: Path<String>, db: Data<Database>) -> HttpResponse {
    let leaves = LeaveACM::get_user_leaves(
        &db.0,
        crate::users::EmpID {
            Emp_Id: id.to_string(),
        },
    )
    .await;

    match leaves {
        Ok(leave) => HttpResponse::Ok().json(leave),
        Err(e) => HttpResponse::BadRequest().json(ErrMsg {
            err_msg: e.to_string(),
        }),
    }
}
