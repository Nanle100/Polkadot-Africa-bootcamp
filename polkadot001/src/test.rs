// Tests for balances module
mod balances_tests {
    use crate::balances::Pallet;

    #[test]
    fn init_balances() {
        let mut balances = Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = Pallet::new();

        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 1),
            Err("Not enough balance")
        );

        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 51),
            Ok(())
        );

        assert_eq!(balances.balance(&"alice".to_string()), 49);
        assert_eq!(balances.balance(&"bob".to_string()), 51);

        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 51),
            Err("Not enough balance")
        );
    }
}

// Tests for system module
mod system_tests {
    use crate::system::Pallet;

    #[test]
    fn system_pallet_work() {
        // Arrange
        // create system pallet
        let mut system_pallet = Pallet::new();

        // Act
        // increase current block number
        system_pallet.inc_block_number();
        // increase the nonce of a user - `Temi`
        system_pallet.inc_nonce(&"Temi".to_string());

        // Assert
        // Check the block number (i.e. 1)
        assert_eq!(system_pallet.block_number(), 1);
        // Check the nonce of Temi (i.e. 1)
        assert_eq!(system_pallet.nonce.get("Temi"), Some(&1));
        // Check the nonce of Faithful (i.e. 0)
        assert_eq!(system_pallet.nonce.get("Faithful"), None);
    }
}