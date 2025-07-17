use std::collections::BTreeMap;

#[derive(Debug)]
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

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        sender: String,
        receiver: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let tx_fee: u128 = 1;
        let total_required = amount.checked_add(tx_fee).ok_or("Overflow")?;

        self.accumulated_fees = self.accumulated_fees.checked_add(tx_fee).ok_or("Overflow")?;

        let sender_balance = self.balance(&sender);
        let receiver_balance = self.balance(&receiver);

        let new_sender_balance = sender_balance
            .checked_sub(total_required)
            .ok_or("Not enough balance")?;
        let new_receiver_balance = receiver_balance.checked_add(amount).ok_or("Overflow")?;

        self.balances.insert(sender, new_sender_balance);
        self.balances.insert(receiver, new_receiver_balance);
        Ok(())
    }
}
