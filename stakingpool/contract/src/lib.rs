use near_sdk::{ext_contract};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env,Gas, near_bindgen};


#[ext_contract(ext_lightency_reward)]
pub trait Lightencyreward {
    fn add_staker(&mut self, account:String, amount:u128);
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StakingPoolContract {

}

impl Default for StakingPoolContract {
    fn default() -> Self {
        panic!("Contract is not initialized yet")
    }
}

// Implement the contract structure
// To be implemented in the front end 
#[near_bindgen]
impl StakingPoolContract {

    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
        }
    }
    // add staker 
    pub fn add_staker(&mut self, account:String, amount:u128) {
        let contract = "rewarder_contract.testnet".to_string().try_into().unwrap();
        ext_lightency_reward::ext(contract)
            .with_static_gas(Gas(5 * 1000000000000))
            .add_staker(account,amount);
    }
}