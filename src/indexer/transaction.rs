use crate::models::transaction::{AccountBalance, AccountType, Transaction};
use solana_transaction_status::{EncodedTransaction, UiConfirmedBlock, UiMessage};

pub fn get_transactions_from_block(block: UiConfirmedBlock) -> Result<Vec<Transaction>, String> {
    let mut transactions = Vec::new();


    let txns = block.transactions.unwrap_or_else(|| vec![]);
    for txn in txns {
        let tx_meta = match txn.meta {
            Some(meta) => meta,
            None => continue, // Skip transactions without metadata
        };

        match txn.transaction {
            EncodedTransaction::Json(data) => {
                let signature = data.signatures[0].to_string();
                let message = data.message;
                let pre_balances = tx_meta.pre_balances;
                let post_balances = tx_meta.post_balances;

                let mut accounts = Vec::new();

                for i in 0..pre_balances.len() {
                    let balance_change = (post_balances[i] as i64) - (pre_balances[i] as i64);

                    if balance_change != 0 {
                        let account_type = if i == 0 {
                            AccountType::FeePayer
                        } else if balance_change < 0 {
                            AccountType::Sender
                        } else {
                            AccountType::Receiver
                        };

                        let address = match &message {
                            UiMessage::Parsed(msg) => {
                                if i < msg.account_keys.len() {
                                    msg.account_keys[i].pubkey.to_string()
                                } else {
                                    "unknown".to_string()
                                }
                            }
                            UiMessage::Raw(msg) => {
                                if i < msg.account_keys.len() {
                                    msg.account_keys[i].to_string()
                                } else {
                                    "unknown".to_string()
                                }
                            }
                        };

                        accounts.push(AccountBalance {
                            address,
                            pre_balance: pre_balances[i] as i64,
                            post_balance: post_balances[i] as i64,
                            balance_change,
                            account_type,
                        });
                    }
                }

                let transaction = Transaction {
                    signature,
                    block_height: block.block_height.map(|height| height as i64),
                    block_time: block.block_time,
                    block_hash: block.blockhash.clone(),
                    success: tx_meta.err.is_none(),
                    fee: tx_meta.fee as i64,
                    accounts,
                };

                transactions.push(transaction);
            }
            _ => continue, // Skip other transaction types
        }
    }

    Ok(transactions)
}
