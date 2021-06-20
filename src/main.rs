mod domain;
mod service;
mod storage;

use std::env;

use service::read_from_file;
use storage::{AccountingStorage, TransactionStorage};

#[async_std::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut accounting_storage = AccountingStorage::new();
    let mut transaction_storage = TransactionStorage::new();

    if let Err(error) =
        read_from_file(filename, &mut accounting_storage, &mut transaction_storage).await
    {
        println!("{}", error);
    }

    if let Err(error) = accounting_storage.print_clients().await {
        println!("{}", error);
    }
}
