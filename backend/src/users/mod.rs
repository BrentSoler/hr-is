use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;

use crate::{
    config::{errors::Errors, Db},
    jwt::Jwt,
};

pub mod get_info;
pub mod login;

#[derive(TS, FromRow, Serialize, Deserialize)]
#[ts(export)]
pub struct Employee {
    Emp_Id: String,
    Emp_Last: String,
    Emp_First: String,
    Emp_Mid: Option<String>,
    Emp_Dept: String,
    Emp_Loc: String,
    Emp_Pswd: Vec<u8>,
}

#[derive(TS, FromRow, Serialize, Deserialize)]
#[ts(export)]
pub struct Schedule {
    Sch_Day: String,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct EmployeeInfo {
    Employee: Employee,
    Access: Vec<Access>,
    Schedules: Vec<Schedule>,
}

#[derive(FromRow, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct Access {
    Acc_Emp: String,
    Mnu_Desc: String,
    Mnu_Http: Option<String>,
}

#[derive(TS, Deserialize)]
#[ts(export)]
pub struct EmployeeLogin {
    Emp_Id: String,
    Emp_Pswd: String,
}

#[derive(TS, Default, Serialize, Deserialize)]
#[ts(export)]
pub struct EmpToken {
    Token: String,
}

#[derive(TS, FromRow, Serialize, Deserialize)]
#[ts(export)]
pub struct EmpID {
    pub Emp_Id: String,
}

pub struct EmployeeACM;

impl EmployeeACM {
    pub async fn login(db: &Db, emp_form: EmployeeLogin, key: String) -> Result<EmpToken, Errors> {
        let user = sqlx::query_as::<_, EmpID>(
            "SELECT Emp_Id FROM employee WHERE Emp_Id=? AND Emp_Pswd=?;",
        )
        .bind(emp_form.Emp_Id)
        .bind(emp_form.Emp_Pswd)
        .fetch_optional(db)
        .await?;

        return match user {
            Some(emp) => match Jwt::build(emp, key) {
                Ok(Token) => Ok(EmpToken { Token }),
                Err(err) => Err(Errors::JwtError(err)),
            },
            None => Err(Errors::RowNotFound("User")),
        };
    }

    pub async fn get_user_and_access(
        db: &Db,
        token: String,
        key: String,
    ) -> Result<EmployeeInfo, Errors> {
        let decoded_token = Jwt::decode(&token, key, db)
            .await
            .map_err(|e| Errors::JwtError(e))?;

        let user_info = sqlx::query_as::<_, Employee>("SELECT * FROM employee WHERE Emp_Id=?")
            .bind(&decoded_token.Emp_Id)
            .fetch_one(db)
            .await?;

        let user_access = sqlx::query_as::<_,Access>("SELECT access.Acc_Emp,menu.Mnu_Desc,menu.Mnu_Http FROM access LEFT JOIN menu ON access.acc_menu=menu.mnu_id WHERE access.acc_emp=?")
            .bind(&decoded_token.Emp_Id).fetch_all(db).await?;

        let user_res = sqlx::query_as::<_, Schedule>(
            "SELECT Sch_Day FROM schedule WHERE sch_emp = ? AND schedule.sch_rest = 1;",
        )
        .bind(&decoded_token.Emp_Id)
        .fetch_all(db)
        .await?;

        return Ok(EmployeeInfo {
            Employee: user_info,
            Access: user_access,
            Schedules: user_res,
        });
    }
}
