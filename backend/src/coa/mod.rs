use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;

use crate::{
    config::{errors::Errors, Db},
    id_builder::IdBuilder,
    users::EmpID,
};

pub mod cancel_coa;
pub mod get_coa;
pub mod get_types;
pub mod post_coa;
pub mod update_coa;

#[derive(Deserialize, Serialize, TS, FromRow)]
#[ts(export)]
pub struct COAView {
    coa: Vec<COASummarywDetails>,
}

#[derive(Deserialize, Serialize, TS, FromRow)]
#[ts(export)]
pub struct COASummary {
    coa_sid: String,
    coa_stype: String,
    coa_tdesc: String,
    coa_stypedetail: String,
    coa_sreason: String,
    coa_semp: String,
    coa_sstatus: i32,
    coa_sapplieddate: NaiveDate,
    coa_sapprovedby: Option<String>,
    coa_sapprovedate: Option<NaiveDate>,
    coa_logdate: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, TS, FromRow)]
#[ts(export)]
pub struct COASummarywDetails {
    coa_sid: String,
    coa_stype: String,
    coa_sapplieddate: NaiveDate,
    coa_tdesc: String,
    coa_stypedetail: String,
    coa_sreason: String,
    coa_semp: String,
    coa_sstatus: i32,
    coa_sapprovedby: Option<String>,
    coa_sapprovedate: Option<NaiveDate>,
    coa_logdate: Option<DateTime<Utc>>,
    details: Vec<COADetail>,
}

#[derive(Deserialize, Serialize, TS, FromRow)]
#[ts(export)]
pub struct COAInsert {
    coa_stype: String,
    coa_tdesc: String,
    coa_sreason: String,
    coa_semp: String,
    details: Vec<COADetailInsert>,
}

#[derive(Deserialize, Serialize, TS, FromRow)]
#[ts(export)]
pub struct COADetail {
    coa_did: String,
    coa_dpk: String,
    coa_dctr: i32,
    coa_dtype: String,
    coa_ddate: NaiveDate,
    coa_dtime: NaiveTime,
}

#[derive(Deserialize, Clone, Serialize, TS, FromRow)]
#[ts(export)]
pub struct COADetailInsert {
    coa_dtype: String,
    coa_ddate: NaiveDate,
    coa_dtime: NaiveTime,
}

#[derive(Deserialize, Clone, Serialize, TS, FromRow)]
#[ts(export)]
pub struct COAUpdate {
    coa_sid: String,
    coa_stype: String,
    coa_tdesc: String,
    coa_sreason: String,
    coa_semp: String,
    details: Vec<COADetailInsert>,
}

#[derive(Debug, Deserialize, Serialize, TS, FromRow)]
#[ts(export)]
pub struct COAType {
    coa_tid: String,
    coa_tdesc: String,
    coa_ttag: i32,
}

#[derive(Debug, Deserialize, Serialize, TS, FromRow)]
#[ts(export)]
pub struct COATypeView {
    types: Vec<COAType>,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct CancelCOA {
    coa_sid: String,
    emp_id: String,
    status: u32,
}

pub struct COAACM;

impl COAACM {
    pub async fn get_coa_type(db: &Db) -> Result<COATypeView, Errors> {
        let types =
            sqlx::query_as::<_, COAType>("SELECT coa_tid,coa_tdesc,coa_ttag FROM coa_type;")
                .fetch_all(db)
                .await?;

        return Ok(COATypeView { types });
    }

    pub async fn get_all(db: &Db, emp_id: String) -> Result<Vec<COASummary>, Errors> {
        let summary = sqlx::query_as::<_,COASummary>("SELECT 
                coa_summary.coa_sid,
                coa_summary.coa_stype,
                coa_type.coa_tdesc,
                coa_summary.coa_stypedetail,
                coa_summary.coa_sapplieddate,
                coa_summary.coa_sreason,
                coa_summary.coa_semp,
                coa_summary.coa_sstatus,
                coa_summary.coa_sapprovedby,
                coa_summary.coa_sapprovedate,
                coa_summary.coa_logdate 
            FROM coa_summary LEFT JOIN coa_type ON coa_summary.coa_stype = coa_type.coa_tid WHERE coa_summary.coa_semp = ?;")
            .bind(emp_id)
            .fetch_all(db)
            .await?;

        return Ok(summary);
    }

    pub async fn get_user_coa(db: &Db, emp_id: EmpID) -> Result<COAView, Errors> {
        let mut coa = COAView { coa: Vec::new() };

        let summary = COAACM::get_all(db, emp_id.Emp_Id).await?;

        for leave in summary {
            let COASummary {
                coa_sid,
                coa_semp,
                coa_stype,
                coa_stypedetail,
                coa_tdesc,
                coa_sreason,
                coa_sapplieddate,
                coa_logdate,
                coa_sapprovedby,
                coa_sstatus,
                coa_sapprovedate,
            } = leave;

            let details = sqlx::query_as::<_,COADetail>("SELECT coa_did,coa_dpk,coa_dctr,coa_dtype,coa_ddate,coa_dtime FROM coa_detail WHERE coa_did = ?")
                .bind(&coa_sid)
                .fetch_all(db)
                .await?;

            coa.coa.push(COASummarywDetails {
                coa_sid,
                coa_stype,
                coa_tdesc,
                coa_stypedetail,
                coa_sreason,
                coa_semp,
                coa_sapplieddate,
                coa_sstatus,
                coa_sapprovedby,
                coa_sapprovedate,
                coa_logdate,
                details,
            })
        }

        return Ok(coa);
    }

    pub async fn apply_coa(db: &Db, coa_form: COAInsert) -> Result<(), Errors> {
        let counter = sqlx::query(
            "SELECT 
                coa_sid 
            FROM coa_summary WHERE coa_semp = ? AND coa_sapplieddate = ?;",
        )
        .bind(coa_form.coa_semp.clone())
        .bind(Utc::now().date_naive())
        .fetch_all(db)
        .await?
        .len()
            + 1;

        let coa_id = IdBuilder::leave_id(&coa_form.coa_semp, Utc::now().to_string(), counter);

        let summary = COASummary {
            coa_semp: coa_form.coa_semp.to_owned(),
            coa_sid: coa_id.to_owned(),
            coa_sapplieddate: Utc::now().date_naive(),
            coa_sapprovedate: None,
            coa_sstatus: 0,
            coa_sapprovedby: None,
            coa_logdate: None,
            coa_sreason: coa_form.coa_sreason.to_owned(),
            coa_tdesc: coa_form.coa_tdesc.to_owned(),
            coa_stypedetail: coa_form.coa_tdesc.to_owned(),
            coa_stype: coa_form.coa_stype.to_owned(),
        };
        let _ = sqlx::query(
            r#"INSERT INTO coa_summary (
                coa_sid,
                coa_sctr,
                coa_stype,
                coa_stypedetail,
                coa_sreason,
                coa_semp,
                coa_sapplieddate,
                coa_sstatus,
                coa_sapprovedby,
                coa_sapprovedate ) 
                VALUES (?,?,?,?,?,?,?,?,?,?);"#,
        )
        .bind(summary.coa_sid)
        .bind(counter as i32)
        .bind(summary.coa_stype)
        .bind(summary.coa_stypedetail)
        .bind(summary.coa_sreason)
        .bind(summary.coa_semp)
        .bind(summary.coa_sapplieddate)
        .bind(summary.coa_sstatus)
        .bind(summary.coa_sapprovedby)
        .bind(summary.coa_sapprovedate)
        .execute(db)
        .await?;

        for (i, detail) in coa_form.details.into_iter().enumerate() {
            let detail = COADetail {
                coa_did: "".to_string(),
                coa_dpk: format!("{}{}", &coa_id, i),
                coa_dctr: i as i32,
                coa_dtype: detail.coa_dtype,
                coa_ddate: detail.coa_ddate,
                coa_dtime: detail.coa_dtime,
            };

            let _ = sqlx::query(
                "INSERT INTO coa_detail
                  (coa_did,coa_dpk,coa_dctr,coa_ddate,coa_dtype,coa_dtime)
                VALUES (?,?,?,?,?,?)",
            )
            .bind(&coa_id)
            .bind(detail.coa_dpk)
            .bind(detail.coa_dctr)
            .bind(detail.coa_ddate)
            .bind(detail.coa_dtype)
            .bind(detail.coa_dtime)
            .execute(db)
            .await?;
        }

        return Ok(());
    }

    pub async fn cancel_coa(cancel_coa: CancelCOA, db: &Db) -> Result<(), Errors> {
        let _ = sqlx::query("UPDATE coa_summary SET coa_sstatus = 2 WHERE coa_sid=?;")
            .bind(&cancel_coa.coa_sid)
            .execute(db)
            .await?;

        return Ok(());
    }

    pub async fn update_coa(db: &Db, coa_form: COAUpdate) -> Result<(), Errors> {
        let summary = COASummary {
            coa_semp: coa_form.coa_semp.to_owned(),
            coa_sid: coa_form.coa_sid.to_owned(),
            coa_sapplieddate: Utc::now().date_naive(),
            coa_sapprovedate: None,
            coa_sstatus: 0,
            coa_sapprovedby: None,
            coa_logdate: None,
            coa_sreason: coa_form.coa_sreason.to_owned(),
            coa_tdesc: coa_form.coa_tdesc.to_owned(),
            coa_stypedetail: coa_form.coa_tdesc.to_owned(),
            coa_stype: coa_form.coa_stype.to_owned(),
        };
        let _ = sqlx::query(
            r#"UPDATE coa_summary SET 
                coa_sreason = ?,
                coa_stypedetail = ?,
                coa_stype = ? 
            WHERE coa_sid = ?;"#,
        )
        .bind(summary.coa_sreason)
        .bind(summary.coa_stypedetail)
        .bind(summary.coa_stype)
        .bind(summary.coa_sid)
        .execute(db)
        .await?;

        let _ = sqlx::query("DELETE FROM coa_detail WHERE coa_did = ?;")
            .bind(&coa_form.coa_sid)
            .execute(db)
            .await?;

        let mut get_db = db.acquire().await?;

        for (i, detail) in coa_form.details.into_iter().enumerate() {
            let detail = COADetail {
                coa_did: "".to_string(),
                coa_dpk: format!("{}{}", &coa_form.coa_sid, i),
                coa_dctr: i as i32,
                coa_dtype: detail.coa_dtype,
                coa_ddate: detail.coa_ddate,
                coa_dtime: detail.coa_dtime,
            };

            let _ = sqlx::query(
                "INSERT INTO coa_detail
                  (coa_did,coa_dpk,coa_dctr,coa_ddate,coa_dtype,coa_dtime)
                VALUES (?,?,?,?,?,?)",
            )
            .bind(&coa_form.coa_sid)
            .bind(detail.coa_dpk)
            .bind(detail.coa_dctr)
            .bind(detail.coa_ddate)
            .bind(detail.coa_dtype)
            .bind(detail.coa_dtime)
            .execute(&mut get_db)
            .await?;
        }

        return Ok(());
    }
}
