use crate::account::Account;
use crate::transaction::{Transaction, TransactionType::*};
use std::collections::HashMap;

pub struct TransactionHandler {
    accounts: HashMap<u16, Account>,
    transactions: HashMap<u32, Transaction>,
}

impl TransactionHandler {
    pub fn new() -> Self {
        TransactionHandler {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn get_accounts(self) -> HashMap<u16, Account> {
        self.accounts
    }

    pub fn process(&mut self, transaction: Transaction) {
        let transaction_type = transaction.tx_type();
        match transaction_type {
            deposit => {
                if !self.accounts.contains_key(&transaction.client()) {
                    // client has no account, so create one
                    self.accounts
                        .insert(transaction.client(), Account::new(&transaction.client()));
                }
                if let Some(acc) = self.accounts.get_mut(&transaction.client()) {
                    acc.deposit(transaction.amount());
                    self.transactions.insert(transaction.tx(), transaction);
                };
            }
            withdrawal => {
                if let Some(acc) = self.accounts.get_mut(&transaction.client()) {
                    acc.withdraw(transaction.amount());
                    self.transactions.insert(transaction.tx(), transaction);
                } else {
                    eprintln!("Error: acc does not exist");
                }
            }
            dispute => {
                if !self.transactions.contains_key(&transaction.tx()) {
                    eprintln!("Warning: tx does not exist. Ignoring.")
                } else {
                    if let Some(acc) = self.accounts.get_mut(&transaction.client()) {
                        acc.dispute(&transaction);
                    } else {
                        eprintln!("Error: acc does not exist");
                    }
                }
            }
            resolve => {
                if !self.transactions.contains_key(&transaction.tx()) {
                    eprintln!("Warning: tx does not exist. Ignoring.")
                } else {
                    if let Some(acc) = self.accounts.get_mut(&transaction.client()) {
                        acc.resolve(&transaction);
                    } else {
                        eprintln!("Error: acc does not exist");
                    }
                };
            }
            chargeback => {
                if !self.transactions.contains_key(&transaction.tx()) {
                    eprintln!("Warning: tx does not exist. Ignoring.")
                } else {
                    if let Some(acc) = self.accounts.get_mut(&transaction.client()) {
                        acc.chargeback(&transaction);
                    } else {
                        eprintln!("Error: acc does not exist");
                    }
                }
            }
        } // end of match
    } // end of process
}