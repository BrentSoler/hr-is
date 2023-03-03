use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

use crate::config::{errors::ErrMsg, Database};

use super::{COAInsert, COAACM};

#[post("/coa")]
pub async fn post_coa(coa_form: Json<COAInsert>, db: Data<Database>) -> HttpResponse {
    let details = &coa_form.details;

    let insert = COAACM::apply_coa(
        &db.0,
        COAInsert {
            coa_semp: coa_form.coa_semp.to_string(),
            coa_sreason: coa_form.coa_sreason.to_string(),
            coa_stype: coa_form.coa_stype.to_string(),
            coa_tdesc: coa_form.coa_tdesc.to_string(),
            details: details.to_vec(),
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
