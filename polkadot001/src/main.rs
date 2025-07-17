mod balances;
mod system;
mod timestamp;


#[cfg(test)]
mod test;

/// This is our runtime, it allows us to interact with all logic in the system.
#[derive(Debug)]
pub struct Runtime {
    pub system: system::Pallet,
    pub balances: balances::Pallet,
    pub timestamp: timestamp::Timestamp,
}

impl Runtime {
    // Create a new instance of the runtime
    fn new() -> Self {
        Runtime {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            timestamp: timestamp::Timestamp::default(),
        }
    }

    fn advance_block(&mut self, moment: u64) {
        self.system.inc_block_number();
        let milliseconds = moment * 1000;
        let new_time = self.timestamp.get() + milliseconds;
        self.timestamp.set(new_time);
    }
}

fn main() {
    let mut runtime = Runtime::new();

    // Users
    let femi = String::from("Femi");
    let temi = String::from("temi");
    let cheryl = String::from("cheryl");
    let nathaniel = String::from("nathaniel");
    let faith = String::from("faith");

    // give some money - GENSIS Block
    runtime.balances.set_balance(&cheryl, 1000);

    // create a block
    // increase block number
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    // first transaction

    runtime.advance_block(6);
    println!("Block {} at timestamp {}\n", runtime.system.block_number(), runtime.timestamp.get());

    runtime.system.inc_nonce(&cheryl);
    let _res = runtime
        .balances
        .transfer(cheryl.clone(), faith.clone(), 50)
        .map_err(|e| println!("error: {}", e));

    // second transaction
    runtime.system.inc_nonce(&cheryl);
    let _res = runtime
        .balances
        .transfer(cheryl.clone(), nathaniel.clone(), 70)
        .map_err(|e| println!("error: {}", e));

    // Create block 2

    runtime.advance_block(6);
    println!("Block {} at timestamp {}\n", runtime.system.block_number(), runtime.timestamp.get());

    runtime.system.inc_block_number();
    // assert_eq!(runtime.system.block_number(), 2);

    runtime.system.inc_nonce(&cheryl);
    let _res = runtime
        .balances
        .transfer(cheryl.clone(), femi.clone(), 100)
        .map_err(|e| println!("error: {}", e));

    runtime.system.inc_nonce(&femi);
    let _res = runtime
        .balances
        .transfer(femi.clone(), temi.clone(), 100)
        .map_err(|e| println!("error: {}", e));

    // block 3 : should fail
    runtime.advance_block(6);
    println!("Block {} at timestamp {}\n", runtime.system.block_number(), runtime.timestamp.get());

    runtime.system.inc_block_number();
    // assert_eq!(runtime.system.block_number(), 3);

    runtime.system.inc_nonce(&cheryl);
    let _res = runtime
        .balances
        .transfer(cheryl.clone(), nathaniel.clone(), 1200)
        .map_err(|e| println!("error: {}", e));

    println!("{:#?}", runtime);
}