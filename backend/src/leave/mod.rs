use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;

pub mod cancel;
pub mod get_credits;
pub mod get_leaves;
pub mod get_pending;
pub mod post_leave;
pub mod update_leave;

use crate::{
    config::{errors::Errors, Db},
    id_builder::IdBuilder,
    users::EmpID,
};

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
pub struct CancelLeave {
    Leave_Id: String,
    Emp_Id: String,
    Status: u32,
}

#[derive(Serialize, Deserialize, FromRow, TS)]
#[ts(export)]
pub struct Leave {
    Eml_Emp: String,
    Eml_Leave: String,
    Lev_Desc: String,
    Availleave: f32,
    Eml_Leacredit: f32,
}

#[derive(Serialize, Deserialize, FromRow, TS)]
#[ts(export)]
pub struct AvailableLeave {
    Leaves: Vec<Leave>,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LeaveView {
    Leaves: Vec<LeaveSummarywDetais>,
}

#[derive(FromRow, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LeaveSummary {
    Lea_Sid: String,
    Lea_Ctr: i64,
    Lev_Desc: String,
    Lea_Stype: String,
    Lea_Sfrm: NaiveDate,
    Lea_Sto: NaiveDate,
    Lea_Sreason: String,
    Lea_Semp: String,
    Lea_Sapplieddate: NaiveDate,
    Lea_Swithpay: f32,
    Lea_Swithoutpay: f32,
    Lea_Sstatus: i8,
    Lea_Sapprovedby: Option<String>,
    Lea_Sapprovedate: Option<NaiveDate>,
    Lea_Logdate: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LeaveUpdate {
    Lea_Sid: String,
    Emp_Id: String,
    Leave_Id: String,
    Lea_Swithpay: f32,
    Lea_Swithoutpay: f32,
    Date_From: String,
    Date_To: String,
    Reason: String,
    Details: Vec<LeaveDetailInsert>,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LeaveInsert {
    Emp_Id: String,
    Leave_Id: String,
    Date_From: String,
    Date_To: String,
    Lea_Swithpay: f32,
    Lea_Swithoutpay: f32,
    Reason: String,
    Details: Vec<LeaveDetailInsert>,
}

#[derive(Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LeaveDetailInsert {
    Date: String,
    Date_Type: String,
    Am_Pm: Option<String>,
}

#[derive(FromRow, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LeaveSummarywDetais {
    Lea_Sid: String,
    Lea_Ctr: i64,
    Lea_Stype: String,
    Lev_Desc: String,
    Lea_Sfrm: NaiveDate,
    Lea_Sto: NaiveDate,
    Lea_Swithpay: f32,
    Lea_Swithoutpay: f32,
    Lea_Sreason: String,
    Lea_Semp: String,
    Lea_Sapplieddate: NaiveDate,
    Lea_Sstatus: i8,
    Lea_Sapprovedby: Option<String>,
    Lea_Sapprovedate: Option<NaiveDate>,
    Lea_Logdate: Option<DateTime<Utc>>,
    Details: Vec<LeaveDetail>,
}

#[derive(FromRow, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LeaveDetail {
    Lea_Dpk: String,
    Lev_Dctr: i64,
    Lea_Ddate: NaiveDate,
    Lea_Dtype: String,
    Lea_Dampm: Option<String>,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PendingLeaves {
    Available: f32,
    Credits: f32,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetAvailableLeaves {
    Emp_Id: String,
    Leave_Type: String,
}

pub struct LeaveACM;

impl LeaveACM {
    pub async fn get_user_leave_credits(db: &Db, emp_id: EmpID) -> Result<AvailableLeave, Errors> {
        let available = sqlx::query_as::<_,Leave>("SELECT empleave.Eml_Emp,`leave`.Lev_Desc,empleave.Eml_Leave,empleave.Eml_Leacredit-empleave.Eml_Used as Availleave,empleave.Eml_Leacredit FROM empleave LEFT JOIN `leave` ON empleave.eml_leave = `leave`.lev_id WHERE empleave.eml_emp=?;")
            .bind(emp_id.Emp_Id)
            .fetch_all(db)
            .await?;

        return Ok(AvailableLeave { Leaves: available });
    }

    pub async fn get_all(db: &Db, emp_id: String) -> Result<Vec<LeaveSummary>, Errors> {
        let summary = sqlx::query_as::<_,LeaveSummary>("SELECT 
                leave_summary.Lea_Sid,
                leave_summary.Lea_Ctr,
                `leave`.Lev_Desc,
                leave_summary.Lea_Sfrm,
                leave_summary.Lea_Sto,
                leave_summary.Lea_Stype,
                leave_summary.Lea_Swithpay,
                leave_summary.Lea_Swithoutpay,
                leave_summary.Lea_Sreason,
                leave_summary.Lea_Semp,
                leave_summary.Lea_Sapplieddate,
                leave_summary.Lea_Sstatus,
                leave_summary.Lea_Sapprovedby,
                leave_summary.Lea_Sapprovedate,
                leave_summary.Lea_Logdate 
            FROM leave_summary LEFT JOIN `leave` ON leave_summary.lea_stype = `leave`.lev_id WHERE leave_summary.lea_semp = ?;")
            .bind(emp_id)
            .fetch_all(db)
            .await?;

        return Ok(summary);
    }

    pub async fn get_user_leaves(db: &Db, emp_id: EmpID) -> Result<LeaveView, Errors> {
        let mut leaves = LeaveView { Leaves: Vec::new() };

        let summary = LeaveACM::get_all(db, emp_id.Emp_Id).await?;

        let mut get_db = db.acquire().await?;

        for leave in summary {
            let LeaveSummary {
                Lea_Sid,
                Lea_Ctr,
                Lea_Stype,
                Lea_Swithpay,
                Lea_Swithoutpay,
                Lev_Desc,
                Lea_Sfrm,
                Lea_Sto,
                Lea_Sreason,
                Lea_Semp,
                Lea_Sapplieddate,
                Lea_Sstatus,
                Lea_Sapprovedby,
                Lea_Sapprovedate,
                Lea_Logdate,
            } = leave;

            let Details = sqlx::query_as::<_,LeaveDetail>("SELECT Lea_Dpk,Lev_Dctr,Lea_Ddate,Lea_Dtype,Lea_Dampm FROM leave_detail WHERE lea_did = ?")
                .bind(&Lea_Sid)
                .fetch_all(&mut get_db)
                .await?;

            leaves.Leaves.push(LeaveSummarywDetais {
                Lea_Sid,
                Lea_Ctr: Lea_Ctr as i64,
                Lea_Stype,
                Lev_Desc,
                Lea_Swithpay,
                Lea_Swithoutpay,
                Lea_Sfrm,
                Lea_Sto,
                Lea_Sreason,
                Lea_Semp,
                Lea_Sapplieddate,
                Lea_Sstatus,
                Lea_Sapprovedby,
                Lea_Sapprovedate,
                Lea_Logdate,
                Details,
            })
        }

        return Ok(leaves);
    }

    pub async fn apply_leave(db: &Db, leave_form: LeaveInsert) -> Result<(), Errors> {
        let counter = sqlx::query(
            "SELECT 
                Lea_Sid 
            FROM leave_summary WHERE lea_semp = ? AND lea_sapplieddate = ?;",
        )
        .bind(leave_form.Emp_Id.clone())
        .bind(Utc::now().date_naive())
        .fetch_all(db)
        .await?
        .len()
            + 1;

        let leave_id = IdBuilder::leave_id(&leave_form.Emp_Id, Utc::now().to_string(), counter);

        let summary = LeaveSummary {
            Lea_Sid: leave_id.clone(),
            Lea_Ctr: counter as i64,
            Lea_Swithpay: leave_form.Lea_Swithpay as f32,
            Lea_Swithoutpay: leave_form.Lea_Swithoutpay as f32,
            Lev_Desc: leave_form.Leave_Id.clone(),
            Lea_Stype: leave_form.Leave_Id.clone(),
            Lea_Sfrm: NaiveDate::parse_from_str(&leave_form.Date_From, "%Y-%m-%d")?,
            Lea_Sto: NaiveDate::parse_from_str(&leave_form.Date_To, "%Y-%m-%d")?,
            Lea_Sreason: leave_form.Reason,
            Lea_Semp: leave_form.Emp_Id.clone(),
            Lea_Sapplieddate: Utc::now().date_naive(),
            Lea_Sstatus: 0,
            Lea_Sapprovedby: None,
            Lea_Sapprovedate: None,
            Lea_Logdate: None,
        };

        let _ = sqlx::query(
            r#"INSERT INTO leave_summary (
                lea_sid,
                lea_ctr,
                lea_stype,
                lea_sfrm,
                lea_sto,
                lea_sreason,
                lea_semp,
                lea_sapplieddate,
                lea_sstatus,
                lea_sapprovedby,
                lea_sapprovedate,
                lea_swithpay,
                lea_swithoutpay)
                VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?);"#,
        )
        .bind(summary.Lea_Sid)
        .bind(summary.Lea_Ctr)
        .bind(summary.Lev_Desc)
        .bind(summary.Lea_Sfrm)
        .bind(summary.Lea_Sto)
        .bind(summary.Lea_Sreason)
        .bind(summary.Lea_Semp)
        .bind(summary.Lea_Sapplieddate)
        .bind(summary.Lea_Sstatus)
        .bind(summary.Lea_Sapprovedby)
        .bind(summary.Lea_Sapprovedate)
        .bind(summary.Lea_Swithpay)
        .bind(summary.Lea_Swithoutpay)
        .execute(db)
        .await?;

        let mut get_db = db.acquire().await?;

        for (i, detail) in leave_form.Details.into_iter().enumerate() {
            let detail = LeaveDetail {
                Lea_Dpk: format!("{}{}", leave_id.to_owned(), i + 1),
                Lev_Dctr: i as i64 + 1,
                Lea_Ddate: NaiveDate::parse_from_str(&detail.Date, "%Y-%m-%d")?,
                Lea_Dtype: detail.Date_Type,
                Lea_Dampm: detail.Am_Pm,
            };

            let _ = sqlx::query(
                "INSERT INTO leave_detail
                  (lea_did,lea_dpk,lev_dctr,lea_ddate,lea_dtype,lea_dampm)
                VALUES (?,?,?,?,?,?)",
            )
            .bind(&leave_id)
            .bind(detail.Lea_Dpk)
            .bind(detail.Lev_Dctr)
            .bind(detail.Lea_Ddate)
            .bind(detail.Lea_Dtype)
            .bind(detail.Lea_Dampm)
            .execute(&mut get_db)
            .await?;
        }

        return Ok(());
    }

    pub async fn cancel_leave(cancel_leave: CancelLeave, db: &Db) -> Result<(), Errors> {
        let _ = sqlx::query("UPDATE leave_summary SET lea_sstatus = 2 WHERE lea_sid=?;")
            .bind(&cancel_leave.Leave_Id)
            .execute(db)
            .await?;

        return Ok(());
    }

    pub async fn get_available_leaves(
        leave_form: GetAvailableLeaves,
        db: &Db,
    ) -> Result<PendingLeaves, Errors> {
        let summary = sqlx::query_as::<_,LeaveSummary>("SELECT 
                leave_summary.Lea_Sid,
                leave_summary.Lea_Ctr,
                `leave`.Lev_Desc,
                leave_summary.Lea_Sfrm,
                leave_summary.Lea_Sto,
                leave_summary.Lea_Sreason,
                leave_summary.Lea_Swithpay,
                leave_summary.Lea_Swithoutpay,
                leave_summary.Lea_Stype,
                leave_summary.Lea_Semp,
                leave_summary.Lea_Sapplieddate,
                leave_summary.Lea_Sstatus,
                leave_summary.Lea_Sapprovedby,
                leave_summary.Lea_Sapprovedate,
                leave_summary.Lea_Logdate 
            FROM leave_summary LEFT JOIN `leave` ON leave_summary.lea_stype = `leave`.lev_id WHERE leave_summary.lea_semp = ? AND leave_summary.lea_sstatus = ? AND leave_summary.lea_stype = ?;")
            .bind(&leave_form.Emp_Id)
            .bind(0)
            .bind(&leave_form.Leave_Type)
            .fetch_all(db)
            .await?;

        let mut avail_leave: Vec<LeaveDetail> = Vec::new();

        let mut get_db = db.acquire().await?;

        for leave in summary.into_iter() {
            let mut get_dates = sqlx::query_as::<_,LeaveDetail>("SELECT Lea_Dpk,Lev_Dctr,Lea_Ddate,Lea_Dtype,Lea_Dampm FROM leave_detail WHERE lea_did = ?")
                .bind(&leave.Lea_Sid)
                .fetch_all(&mut get_db)
                .await?;

            avail_leave.append(&mut get_dates);
        }

        let available = sqlx::query_as::<_,Leave>("SELECT empleave.Eml_Emp,`leave`.Lev_Desc,empleave.Eml_Leave,empleave.Eml_LeaCredit-empleave.Eml_Used as Availleave,empleave.Eml_Leacredit FROM empleave LEFT JOIN `leave` ON empleave.eml_leave = `leave`.lev_id WHERE empleave.eml_emp=? AND empleave.eml_leave=?;")
            .bind(&leave_form.Emp_Id)
            .bind(&leave_form.Leave_Type)
            .fetch_all(db)
            .await?;

        let credits = &available[0].Availleave;

        let mut pending = 0.;

        for leave in avail_leave.into_iter() {
            if leave.Lea_Dtype == "H" {
                pending += 0.5;
            } else {
                pending += 1.;
            }
        }

        Ok(PendingLeaves {
            Available: credits - pending,
            Credits: if available[0].Availleave <= 0.0 {
                0.0
            } else {
                available[0].Availleave
            },
        })
    }

    pub async fn update_leave(db: &Db, leave_form: LeaveUpdate) -> Result<(), Errors> {
        let summary = LeaveSummary {
            Lea_Sid: leave_form.Lea_Sid.clone(),
            Lea_Ctr: 0,
            Lev_Desc: leave_form.Leave_Id.clone(),
            Lea_Swithpay: leave_form.Lea_Swithpay as f32,
            Lea_Swithoutpay: leave_form.Lea_Swithoutpay as f32,
            Lea_Stype: leave_form.Leave_Id.clone(),
            Lea_Sfrm: NaiveDate::parse_from_str(&leave_form.Date_From, "%Y-%m-%d")?,
            Lea_Sto: NaiveDate::parse_from_str(&leave_form.Date_To, "%Y-%m-%d")?,
            Lea_Sreason: leave_form.Reason,
            Lea_Semp: leave_form.Emp_Id.clone(),
            Lea_Sapplieddate: Utc::now().date_naive(),
            Lea_Sstatus: 0,
            Lea_Sapprovedby: None,
            Lea_Sapprovedate: None,
            Lea_Logdate: None,
        };

        let _ = sqlx::query(
            r#"UPDATE leave_summary SET
                lea_stype = ?,
                lea_sfrm = ?,
                lea_sto = ?,
                lea_sreason = ?,
                lea_swithpay = ?,
                lea_swithoutpay = ?
            WHERE lea_sid = ?;"#,
        )
        .bind(summary.Lev_Desc)
        .bind(summary.Lea_Sfrm)
        .bind(summary.Lea_Sto)
        .bind(summary.Lea_Sreason)
        .bind(summary.Lea_Swithpay)
        .bind(summary.Lea_Swithoutpay)
        .bind(summary.Lea_Sid)
        .execute(db)
        .await?;

        let _ = sqlx::query("DELETE FROM leave_detail WHERE lea_did = ?;")
            .bind(&leave_form.Lea_Sid)
            .execute(db)
            .await?;

        let mut get_db = db.acquire().await?;

        for (i, detail) in leave_form.Details.into_iter().enumerate() {
            let detail = LeaveDetail {
                Lea_Dpk: format!("{}{}", leave_form.Lea_Sid.to_owned(), i + 1),
                Lev_Dctr: i as i64 + 1,
                Lea_Ddate: NaiveDate::parse_from_str(&detail.Date, "%Y-%m-%d")?,
                Lea_Dtype: detail.Date_Type,
                Lea_Dampm: detail.Am_Pm,
            };

            let _ = sqlx::query(
                "INSERT INTO leave_detail
                  (lea_did,lea_dpk,lev_dctr,lea_ddate,lea_dtype,lea_dampm)
                VALUES (?,?,?,?,?,?)",
            )
            .bind(&leave_form.Lea_Sid)
            .bind(detail.Lea_Dpk)
            .bind(detail.Lev_Dctr)
            .bind(detail.Lea_Ddate)
            .bind(detail.Lea_Dtype)
            .bind(detail.Lea_Dampm)
            .execute(&mut get_db)
            .await?;
        }

        return Ok(());
    }
}
