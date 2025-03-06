use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Type;


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Transaction {
    pub signature: String,
    pub block_height: Option<i64>,
    pub block_hash: String,
    pub block_time: Option<i64>,
    pub success: bool,
    pub fee: i64,
    pub accounts: Vec<AccountBalance>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDB {
    pub id : Option<i32>,
    pub signature : String,
    pub block_height : Option<i64>,
    pub block_hash : String,
    pub block_time : Option<i64>,
    pub success : bool,
    pub fee : i64,
    pub created_at : Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct AccountBalance {
    pub address: String,
    pub pre_balance: i64,
    pub post_balance: i64,
    pub balance_change: i64,
    pub account_type: AccountType,
}

#[derive(Debug, Serialize, Deserialize, Clone,Type)]
#[sqlx(type_name = "account_type")]
#[sqlx(rename_all = "lowercase")]
pub enum AccountType {
    FeePayer,
    Sender,
    Receiver,
    Other,
}


impl AccountType {
    pub fn to_string(&self) -> String {
        match self {
            AccountType::FeePayer => "fee_payer".to_string(),
            AccountType::Sender => "sender".to_string(),
            AccountType::Receiver => "receiver".to_string(),
            AccountType::Other => "other".to_string(),
        }
    }
}