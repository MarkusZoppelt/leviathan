use serde::Deserialize;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Deserialize)]
pub enum TransactionType {
    deposit,
    withdrawal,
    dispute,
    resolve,
    chargeback,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub tx_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f32>,

    #[serde(skip_deserializing)]
    pub is_disputed: bool,
}

impl Transaction {
    // Getters for private vars
    pub fn tx_type(&self) -> TransactionType {
        self.tx_type
    }
    pub fn client(&self) -> u16 {
        self.client
    }
    pub fn tx(&self) -> u32 {
        self.tx
    }
    pub fn amount(&self) -> f32 {
        let _amount = match self.amount {
            Some(a) => a,
            None => 0.0,
        };
        _amount
    }
}
