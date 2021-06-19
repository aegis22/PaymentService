use csv::Reader;
use log::debug;
use std::error::Error;

use super::models::Operation;
use super::models::Transaction;
use super::models::TransactionType;

use super::AccountingStorage;
use super::TransactionStorage;

pub async fn read_from_file(
    filename: &str,
    accounting_storage: &mut AccountingStorage,
    transaction_storage: &mut TransactionStorage,
) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(filename)?;

    for result in reader.deserialize() {
        let record: Operation = result?;

        let client = record.client;
        let tx = Transaction {
            tx: record.tx,
            amount: record.amount,
            disputed: false,
        };

        if !accounting_storage.exists_account(client) {
            accounting_storage.create_account(client).await?;
        }

        let tx_string = record.r#type;
        let tx_type: TransactionType = tx_string.parse().expect("Wrong transaction type");
        match tx_type {
            TransactionType::Chargeback => {
                debug!("Attempting to chargeback in client {:?}", client);
                accounting_storage.chargeback(client, tx).await?;
            }
            TransactionType::Deposit => {
                debug!("Attempting to deposit in client {:?}", client);
                accounting_storage.deposit(client, record.amount).await?;
                transaction_storage.add_transaction(tx);
            }
            TransactionType::Dispute => {
                debug!("Attempting to dispute for client {:?}", client);
                // get tx from ID
                accounting_storage.dispute(client, tx).await?;
                // tx.disputed = true;
            }
            TransactionType::Resolve => {
                debug!("Attempting to resolve from client {:?}", client);
                accounting_storage.resolve(client, tx.tx).await?;
            }
            TransactionType::Withdrawal => {
                debug!("Attempting to withdraw from client {:?}", client);
                accounting_storage.withdraw(client, record.amount).await?;
                transaction_storage.add_transaction(tx);
            }
        }
    }

    Ok(())
}