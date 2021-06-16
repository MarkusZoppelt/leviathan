#[cfg(test)]
mod tests {

    use crate::account::Account;
    use crate::transaction::TransactionType::withdrawal;
    use crate::Transaction;

    #[test]
    fn test_deposit() {
        let mut acc = Account::new(&99);

        acc.deposit(42.1337);

        assert_eq!(acc.available(), 42.1337);
        assert_eq!(acc.total(), 42.1337);
    }

    #[test]
    fn test_withdraw() {
        let mut acc = Account::new(&99);

        acc.deposit(42.1337);
        acc.withdraw(42.0);

        assert_eq!(acc.available(), 0.1337);
        assert_eq!(acc.total(), 0.1337);
    }

    #[test]
    fn test_dispute() {
        let mut acc = Account::new(&99);

        acc.deposit(42.1337);

        let transaction = Transaction {
            tx_type: withdrawal,
            client: 99,
            tx: 999,
            amount: Some(0.1337),
            is_disputed: false
        };

        acc.dispute(&transaction);

        // available should decrase
        // held should be tx.amount and
        // total should stay the same
        assert_eq!(acc.available(), 42.0);
        assert_eq!(acc.held(), 0.1337);
        assert_eq!(acc.total(), 42.1337);
    }

    #[test]
    fn test_resolve() {
        let mut acc = Account::new(&99);

        acc.deposit(42.1337);

        let transaction = Transaction {
            tx_type: withdrawal,
            client: 99,
            tx: 999,
            amount: Some(0.1337),
            is_disputed: false
        };

        acc.dispute(&transaction);
        acc.resolve(&transaction);

        // held funds should decrease by the amount disputed
        // available should increase by the amount disputed
        // total should remain the same.
        assert_eq!(acc.available(), 42.1337);
        assert_eq!(acc.held(), 0.0);
        assert_eq!(acc.total(), 42.1337);
    }

    #[test]
    fn test_chargeback() {
        let mut acc = Account::new(&99);

        acc.deposit(42.1337);

        let transaction = Transaction {
            tx_type: withdrawal,
            client: 99,
            tx: 999,
            amount: Some(0.1337),
            is_disputed: false
        };

        acc.dispute(&transaction);
        acc.chargeback(&transaction);

        // held and total should decrease by the amount previously disputed
        assert_eq!(acc.held(), 0.0);
        assert_eq!(acc.total(), 42.);

        // chargeback => locked.
        assert_eq!(acc.locked(), true);
    }
}
