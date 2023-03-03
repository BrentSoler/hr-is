use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::{config::Db, users::EmpID};

pub type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize)]
pub struct Jwt;

impl Jwt {
    pub fn build(token: EmpID, key: String) -> Result<String, &'static str> {
        let jwt_key = HmacSha256::new_from_slice(&key.as_bytes()).map_err(|_| "Invalid Key")?;
        let header: Header = Default::default();

        let token = Token::new(header, token)
            .sign_with_key(&jwt_key)
            .map_err(|_| "Error Building Token")?;

        return Ok(token.into());
    }

    pub async fn decode(token: &str, key: String, db: &Db) -> Result<EmpID, &'static str> {
        let jwt_key = HmacSha256::new_from_slice(&key.as_bytes()).map_err(|_| "Invalid Key")?;

        let user_id: Token<Header, EmpID, _> =
            VerifyWithKey::verify_with_key(token, &jwt_key).map_err(|_| "Token Invalid")?;

        let (_, emp_id) = user_id.into();

        let mut get_db = db.acquire().await.map_err(|_| "Pool Not Available")?;

        let verify_token =
            sqlx::query_as::<_, EmpID>("SELECT Emp_Id FROM employee WHERE Emp_Id=?;")
                .bind(emp_id.Emp_Id)
                .fetch_optional(&mut get_db)
                .await
                .map_err(|_| "Error Fetching User From Token")?;

        return match verify_token {
            Some(user) => Ok(user),
            None => Err("Invalid Token Provided"),
        };
    }
}
