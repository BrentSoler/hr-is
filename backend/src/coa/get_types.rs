use actix_web::{get, web::Data, HttpResponse};

use crate::config::{errors::ErrMsg, Database};

use super::COAACM;

#[get("/coa/types/")]
pub async fn get_types(db: Data<Database>) -> HttpResponse {
    let types = COAACM::get_coa_type(&db.0).await;

    match types {
        Ok(types) => HttpResponse::Ok().json(types),
        Err(e) => HttpResponse::BadRequest().json(ErrMsg {
            err_msg: e.to_string(),
        }),
    }
}
