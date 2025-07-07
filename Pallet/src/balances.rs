use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
    accumulated_fees: u128,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
            accumulated_fees: 0,
        }
    }

    pub fn set_balance(&mut self, user: &String, balance: u128) {
        self.balances.insert(user.clone(), balance);
    }

    pub fn check_balance(&self, user: &String) -> u128 {
        *self.balances.get(user).unwrap_or(&0)
    }

    pub fn transfer(&mut self, sender: &String, receiver: &String, amount: u128) -> Result<(), &'static str>{
        let tx_fee: u128 = 1;
        let total_required = amount.checked_add(tx_fee).ok_or("Overflow")?;

        self.accumulated_fees = self.accumulated_fees.checked_add(tx_fee).ok_or("Overflow")?;

        let sender_balance = self.check_balance(sender);
        let receiver_balance = self.check_balance(receiver);


        let new_sender_balance= sender_balance.checked_sub(total_required).ok_or("Not enough balance")?;
        let new_receiver_balance = receiver_balance.checked_add(amount).ok_or("Overflow")?;

        self.set_balance(sender, new_sender_balance);
        self.set_balance(receiver, new_receiver_balance);
        Ok(())
    }

    pub fn get_accumulated_fees(&self) -> u128 {
        self.accumulated_fees
    }
}







#[cfg(test)]
mod test{
    use crate::balances::Pallet;
     #[test]
    fn init_balance(){
        let mut balances = Pallet::new();

        balances.set_balance(&"Alice".to_string(), 100);
        assert_eq!(balances.check_balance(&"Alice".to_string()), 100);
        assert_eq!(balances.check_balance(&"Bob".to_string()), 0);
    }
     #[test]
    fn transfer_test(){
        let mut balances = Pallet::new();

        balances.set_balance(&"Alice".to_string(), 100);
        balances.set_balance(&"Bob".to_string(), 50);

        assert_eq!(balances.transfer(&"Alice".to_string(), &"Bob".to_string(), 30), Ok(()));
        assert_eq!(balances.check_balance(&"Alice".to_string()), 69);
        assert_eq!(balances.check_balance(&"Bob".to_string()), 80);

        assert_eq!(balances.transfer(&"Alice".to_string(), &"Bob".to_string(), 100), Err("Not enough balance"));
        assert_eq!(balances.get_accumulated_fees(), 2);


    }
}