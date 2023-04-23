use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
pub struct MyContract {
    // define contract state here
}

impl MyContract {
    // define contract methods here
    
    pub fn create_account(&mut self, account_id: AccountId) {
        let initial_balance: u128 = 0;
        let new_account = env::signer_account_pk().create_account(account_id);
        new_account.transfer(initial_balance);
    }
}
