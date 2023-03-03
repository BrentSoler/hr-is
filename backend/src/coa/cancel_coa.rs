use actix_web::{
    patch,
    web::{Data, Json},
    HttpResponse,
};

use crate::config::{errors::ErrMsg, Database};

use super::{CancelCOA, COAACM};

#[patch("/coa")]
pub async fn cancel_coa(db: Data<Database>, cancel_coa: Json<CancelCOA>) -> HttpResponse {
    let cancel = COAACM::cancel_coa(
        CancelCOA {
            coa_sid: cancel_coa.coa_sid.to_string(),
            emp_id: cancel_coa.emp_id.to_string(),
            status: cancel_coa.status,
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
