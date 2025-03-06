-- Add migration script here
CREATE TABLE IF NOT EXISTS transaction (
    id SERIAL PRIMARY KEY,
    signature VARCHAR(255) NOT NULL UNIQUE,
    block_height BIGINT NOT NULL,
    block_hash TEXT NOT NULL,
    block_time BIGINT NOT NULL,
    success BOOLEAN NOT NULL,
    fee BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_transaction_signature ON transaction(signature);
CREATE INDEX IF NOT EXISTS idx_transaction_block_time ON transaction(block_time);
CREATE INDEX IF NOT EXISTS idx_transaction_block_height ON transaction(block_height);
CREATE INDEX IF NOT EXISTS idx_transaction_block_hash ON transaction(block_hash);


-- Create account_balance table
CREATE TABLE IF NOT EXISTS account_balance (
    id SERIAL PRIMARY KEY,
    address VARCHAR(255) NOT NULL,
    pre_balance BIGINT NOT NULL,
    post_balance BIGINT NOT NULL,
    balance_change BIGINT NOT NULL,
    account_type VARCHAR(255) NOT NULL,
    transaction_id INTEGER NOT NULL,
    FOREIGN KEY (transaction_id) REFERENCES transaction(id)
);


CREATE INDEX IF NOT EXISTS idx_account_balance_address ON account_balance(address);
CREATE INDEX IF NOT EXISTS idx_account_balance_transaction_id ON account_balance(transaction_id);