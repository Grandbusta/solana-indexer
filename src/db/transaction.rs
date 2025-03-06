use crate::models::transaction::{AccountBalance, Transaction, TransactionDB};
use sqlx::{Error, Postgres};

pub async fn save_transaction<'a>(
    transaction: Transaction,
    tx: &mut sqlx::Transaction<'a, Postgres>,
) -> Result<TransactionDB, Error> {
    let record = sqlx::query_as!(
        TransactionDB,
        r#"
        INSERT INTO transaction (
            signature, block_height, block_time, block_hash, success, fee, created_at
        ) VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP)
        RETURNING id, signature, block_height, block_time, block_hash, success, fee, created_at
        "#,
        transaction.signature,
        transaction.block_height,
        transaction.block_time,
        transaction.block_hash,
        transaction.success,
        transaction.fee
    )
    .fetch_one(&mut **tx)
    .await?;
    Ok(record)
}

pub async fn save_accounts<'a>(
    accounts: Vec<AccountBalance>,
    transaction_id: i32,
    tx: &mut sqlx::Transaction<'a, Postgres>,
) -> Result<(), Error> {
    for account in accounts {
        sqlx::query!(
            r#"
            INSERT INTO account_balance (
                address, pre_balance, post_balance, balance_change, account_type, transaction_id
            ) VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            account.address,
            account.pre_balance as i64,
            account.post_balance as i64,
            account.balance_change,
            account.account_type.to_string(),
            transaction_id
        )
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}
