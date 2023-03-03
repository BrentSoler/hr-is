use actix_web::{
    put,
    web::{Data, Json},
    HttpResponse,
};

use crate::config::{errors::ErrMsg, Database};

use super::{COAUpdate, COAACM};

#[put("/coa")]
pub async fn update_coa(coa_form: Json<COAUpdate>, db: Data<Database>) -> HttpResponse {
    let details = &coa_form.details;

    let insert = COAACM::update_coa(
        &db.0,
        COAUpdate {
            coa_sid: coa_form.coa_sid.to_string(),
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
