pub mod bank {
    pub struct Account {
        pub id: u32,
        balance: i64,
    }

    impl Account {
        pub fn new(id: u32, initial_balance: i64) -> Self {
            Account {
                id,
                balance: initial_balance,
            }
        }

        pub fn balance(&self) -> i64 {
            self.balance
        }

        pub fn deposit(&mut self, amount: i64) {
            self.balance += amount;
        }

        pub fn withdraw(&mut self, amount: i64) -> bool {
            if amount > self.balance {
                return false;
            };

            self.balance -= amount;
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bank::Account;

    #[test]
    fn test_new_account() {
        let acc = Account::new(1, 1000);
        assert_eq!(acc.id, 1);
        assert_eq!(acc.balance(), 1000);
    }

    #[test]
    fn test_deposit() {
        let mut acc = Account::new(1, 1000);
        acc.deposit(500);
        assert_eq!(acc.balance(), 1500);
    }

    #[test]
    fn test_withdraw_success() {
        let mut acc = Account::new(1, 1000);
        assert!(acc.withdraw(300));
        assert_eq!(acc.balance(), 700);
    }

    #[test]
    fn test_withdraw_insufficient() {
        let mut acc = Account::new(1, 1000);
        assert!(!acc.withdraw(1500));
        assert_eq!(acc.balance(), 1000);
    }

    #[test]
    fn test_withdraw_exact() {
        let mut acc = Account::new(1, 1000);
        assert!(acc.withdraw(1000));
        assert_eq!(acc.balance(), 0);
    }

    #[test]
    fn test_multiple_operations() {
        let mut acc = Account::new(42, 0);
        acc.deposit(1000);
        acc.deposit(500);
        assert!(acc.withdraw(200));
        assert_eq!(acc.balance(), 1300);
    }
}
