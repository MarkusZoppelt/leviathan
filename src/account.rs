#![allow(dead_code)]
// allow dead code for tests

use crate::transaction::Transaction;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Account {
    client: u16,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}

// ugly but works, maybe find a better way...
fn round_number(n: f32) -> f32 {
    let x = (n * 10000.0).round() / 10000.0;
    x
}

impl Account {
    pub fn new(client_id: &u16) -> Account {
        Account {
            client: client_id.to_owned(),
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }

    // Getters for private vars, used for test
    pub fn client(&self) -> u16 {
        self.client
    }
    pub fn available(&self) -> f32 {
        self.available
    }
    pub fn held(&self) -> f32 {
        self.held
    }
    pub fn total(&self) -> f32 {
        self.total
    }
    pub fn locked(&self) -> bool {
        self.locked
    }

    pub async fn deposit(&mut self, amount: f32) -> bool {
        // FYI: allowing users to deposit even when locked :P

        let _x = round_number(self.available + amount);
        self.available = _x;
        self.total = _x;
        true
    }

    pub async fn withdraw(&mut self, amount: f32) -> bool {
        if self.locked() {
            return false;
        }

        if self.available - amount > 0.0 {
            let _x = round_number(self.available - amount);

            self.available = _x;
            self.total = _x;
            true
        } else {
            false
        }
    }

    pub async fn dispute(&mut self, tx: &Transaction) -> bool {
        if self.locked() {
            return false;
        }

        let amount = tx.amount();

        self.available = round_number(self.available - amount);
        self.held = round_number(self.held + amount);

        false
    }

    pub async fn resolve(&mut self, tx: &Transaction) -> bool {
        if self.locked() {
            return false;
        }

        let amount = tx.amount();

        self.held = round_number(self.held - amount);
        self.available = round_number(self.available + amount);

        true
    }

    pub async fn chargeback(&mut self, tx: &Transaction) -> bool {
        if self.locked() {
            return false;
        }

        let amount = tx.amount();
        self.held = round_number(self.held - amount);
        self.total = round_number(self.total - amount);
        self.locked = true;

        true
    }

    pub async fn lock(&mut self) {
        self.locked = true;
    }

    pub async fn unlock(&mut self) {
        self.locked = false;
    }
}