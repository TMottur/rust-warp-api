use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub exp: DateTime<Utc>,
    pub account_id: AccountId,
    pub nbf: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub id: Option<AccountId>,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId(pub i32);
