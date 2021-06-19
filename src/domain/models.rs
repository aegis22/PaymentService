use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum TransactionType {
    Chargeback,
    Deposit,
    Dispute,
    Resolve,
    Withdrawal,
}

impl FromStr for TransactionType {
    type Err = ();

    fn from_str(input: &str) -> Result<TransactionType, Self::Err> {
        let lowercase = input.to_lowercase();
        match lowercase.as_str() {
            "chargeback" => Ok(TransactionType::Chargeback),
            "deposit" => Ok(TransactionType::Deposit),
            "dispute" => Ok(TransactionType::Dispute),
            "resolve" => Ok(TransactionType::Resolve),
            "withdrawal" => Ok(TransactionType::Withdrawal),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Operation {
    pub r#type: String,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>,
}

#[derive(Debug)]
pub struct Transaction {
    pub tx: u32,
    pub amount: f64,
    pub disputed: bool,
}

#[derive(Debug, Serialize)]
pub struct Account {
    pub client: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}
