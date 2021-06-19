use float_cmp::approx_eq;
use thiserror::Error;

use super::models::{Account, Transaction};

#[derive(Error, Debug)]
pub enum AccountingError {
    #[error("account not found")]
    AccountNotFound,
    #[error("Not enough funds")]
    NotEnoughFunds,
    #[error("csv error")]
    CsvError(#[from] csv::Error),
}

pub type AccountingResponse<T> = Result<T, AccountingError>;

#[derive(Debug)]
pub struct AccountingStorage {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}

impl AccountingStorage {
    pub fn new() -> AccountingStorage {
        AccountingStorage {
            accounts: Vec::new(),
            transactions: Vec::new(),
        }
    }

    pub async fn create_account(&mut self, client: u16) -> AccountingResponse<()> {
        let new_account = Account {
            client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        };

        self.accounts.push(new_account);

        Ok(())
    }

    pub async fn get_account(&mut self, client: u16) -> Option<&mut Account> {
        self.accounts
            .iter_mut()
            .find(|account| account.client == client)
    }

    pub fn exists_account(&self, client: u16) -> bool {
        self.accounts.iter().any(|account| account.client == client)
    }

    pub async fn chargeback(&mut self, client: u16, amount: f64) -> AccountingResponse<()> {
        let acc = self.get_account(client).await;
        match acc {
            Some(mut account) => {
                account.held -= amount;
                account.total -= amount;
                account.locked = true;
            }
            None => return Err(AccountingError::AccountNotFound),
        }

        Ok(())
    }

    pub async fn deposit(&mut self, client: u16, amount: f64) -> AccountingResponse<()> {
        let acc = self.get_account(client).await;
        match acc {
            Some(mut account) => {
                if !account.locked {
                    account.available += amount;
                    account.total += amount;
                }
                Ok(())
            }
            None => Err(AccountingError::AccountNotFound),
        }
    }

    pub async fn dispute(&mut self, client: u16, amount: f64) -> AccountingResponse<()> {
        let acc = self.get_account(client).await;
        match acc {
            Some(mut account) => {
                if !account.locked {
                    account.available -= amount;
                    account.held += amount;
                }
                Ok(())
            }
            None => Err(AccountingError::AccountNotFound),
        }
    }

    pub async fn resolve(&mut self, client: u16, amount: f64) -> AccountingResponse<()> {
        let acc = self.get_account(client).await;
        match acc {
            Some(mut account) => {
                if !account.locked {
                    account.held -= amount;
                    account.available += amount;
                }
                Ok(())
            }
            None => Err(AccountingError::AccountNotFound),
        }
    }

    pub async fn withdraw(&mut self, client: u16, amount: f64) -> AccountingResponse<()> {
        let acc = self.get_account(client).await;
        match acc {
            Some(mut account) => {
                if !account.locked {
                    let available = account.available;
                    if approx_eq!(f64, available, amount) || available - amount > 0.0 {
                        account.available -= amount;
                        account.total -= amount;
                    } else {
                        return Err(AccountingError::NotEnoughFunds);
                    }
                }
            }
            None => return Err(AccountingError::AccountNotFound),
        }

        Ok(())
    }

    pub async fn print_clients(&self) -> AccountingResponse<()> {
        let mut writer = csv::Writer::from_writer(std::io::stdout());

        for account in self.accounts.iter() {
            writer.serialize(account)?;
        }

        if let Err(error) = writer.flush() {
            return Err(AccountingError::CsvError(error.into()));
        };

        Ok(())
    }
}
