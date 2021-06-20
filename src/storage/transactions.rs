use crate::domain::models::Transaction;

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

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.transactions.push(tx);
    }

    pub fn get_transaction(&mut self, tx: u32) -> Option<&mut Transaction> {
        self.transactions
            .iter_mut()
            .find(|transaction| transaction.tx == tx)
    }
}
