use super::models::Transaction;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("transaction not found")]
    TransactionNotFound,
}

pub type TransactionResponse<T> = Result<T, TransactionError>;

#[derive(Debug)]
pub struct TransactionStorage {
    pub transactions: Vec<Transaction>,
}

impl TransactionStorage {
    pub fn new() -> TransactionStorage {
        TransactionStorage {
            transactions: Vec::new(),
        }
    }

    pub fn exists_transaction(&self, tx: u32) -> bool {
        self.transactions
            .iter()
            .any(|transaction| transaction.tx == tx)
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.transactions.push(tx);
    }

    pub fn get_transaction(&mut self, tx: u32) -> Option<&mut Transaction> {
        self.transactions
            .iter_mut()
            .find(|transaction| transaction.tx == tx)
    }
}
