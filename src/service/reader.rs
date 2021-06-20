use csv::Reader;
use log::debug;
use std::error::Error;

use crate::domain::models::{Operation, Transaction, TransactionType};
use crate::storage::{AccountingStorage, TransactionStorage};

pub async fn read_from_file(
    filename: &str,
    accounting_storage: &mut AccountingStorage,
    transaction_storage: &mut TransactionStorage,
) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(filename)?;

    for result in reader.deserialize() {
        let record: Operation = result?;

        let client = record.client;
        let op_amount = record.amount;

        if !accounting_storage.exists_account(client) {
            accounting_storage.create_account(client).await?;
        }

        let tx_string = record.r#type;
        let tx_type: TransactionType = tx_string.parse().expect("Wrong transaction type");
        match tx_type {
            TransactionType::Chargeback => {
                debug!("Attempting to chargeback in client {:?}", client);
                let option = transaction_storage.get_transaction(record.tx);
                if let Some(tx) = option {
                    if tx.disputed {
                        accounting_storage.chargeback(client, tx.amount).await?;
                    }
                }
            }
            TransactionType::Deposit => {
                debug!("Attempting to deposit in client {:?}", client);
                if let Some(amount) = op_amount {
                    accounting_storage.deposit(client, amount).await?;
                    let tx = Transaction {
                        tx: record.tx,
                        amount,
                        disputed: false,
                    };
                    transaction_storage.add_transaction(tx);
                }
            }
            TransactionType::Dispute => {
                debug!("Attempting to dispute for client {:?}", client);
                let option = transaction_storage.get_transaction(record.tx);
                if let Some(tx) = option {
                    accounting_storage.dispute(client, tx.amount).await?;
                    tx.disputed = true;
                }
            }
            TransactionType::Resolve => {
                debug!("Attempting to resolve from client {:?}", client);
                let option = transaction_storage.get_transaction(record.tx);
                if let Some(tx) = option {
                    if tx.disputed {
                        accounting_storage.resolve(client, tx.amount).await?;
                    }
                }
            }
            TransactionType::Withdrawal => {
                debug!("Attempting to withdraw from client {:?}", client);
                if let Some(amount) = op_amount {
                    accounting_storage.withdraw(client, amount).await?;
                    let tx = Transaction {
                        tx: record.tx,
                        amount: -amount,
                        disputed: false,
                    };
                    transaction_storage.add_transaction(tx);
                }
            }
        }
    }

    Ok(())
}
