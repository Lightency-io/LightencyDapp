use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ext_contract, Promise, PromiseError};
use near_sdk::{env, near_bindgen, Gas};

pub const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(ext_pool)]
pub trait Stakingpool {
    fn transfer_lts (&mut self, amount:u128);
}

#[ext_contract(ext_ft)]
pub trait Rewarder {
    #[payable]
    fn add_staker(&mut self, account: String, amount: u128);
    fn unstake (&mut self, account:String, amount: u128);
    fn withdraw(&mut self, account:String, amount:u128);
}

#[ext_contract(ext_lts)]
pub trait Lts {
    fn ft_transfer (&mut self, receiver_id:String, amount:String, memo:String);
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StakingContract {

}

// Define the default, which automatically initializes the contract
impl Default for StakingContract {
    fn default() -> Self {
        panic!("Contract is not initialized yet")
    }
}

// Implement the contract structure
// To be implemented in the front end
#[near_bindgen]
impl StakingContract {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {

        }
    }

    // Methods.

    // stake function 
    pub fn stake(&self, amount: u128) -> Promise {
        let account_lts= "light-token.testnet".to_string().try_into().unwrap();
        // Create a promise to call HelloNEAR.get_greeting()
        let promise = ext_lts::ext(account_lts)
        .with_static_gas(Gas(2 * TGAS))
        .with_attached_deposit(1)
        .ft_transfer("lightencypool.testnet".to_string(),(amount*100000000).to_string(),"".to_string());
        
        return promise.then( // Create a promise to callback query_greeting_callback
        Self::ext(env::current_account_id())
        .with_static_gas(Gas(10 * TGAS))
        .staking_callback(env::signer_account_id().to_string(),amount)
        )
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn staking_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>, account:String ,amount: u128) -> Promise {
        let account_reward = "rewarder_contract.testnet".to_string().try_into().unwrap();
        // Check if the promise succeeded
        if call_result.is_err() {
        panic!("There was an error contacting the pool contract");
        }

        // Return the promise
        let p = ext_ft::ext(account_reward)
        .with_static_gas(Gas(2 * TGAS))
        .add_staker(account, amount);
        p
    }

    // unstake function
    pub fn unstake(&mut self, amount: u128) {
        // account as account id
        let account = "rewarder_contract.testnet".to_string().try_into().unwrap();
        let account_pool = "lightencypool.testnet".to_string().try_into().unwrap();
        // cross call function to change data in the reward contract
        ext_ft::ext(account)
            .with_static_gas(Gas(5 * TGAS))
            .unstake(env::signer_account_id().to_string(),amount)
            .and(
                ext_pool::ext(account_pool)
                .with_static_gas(Gas(2 * TGAS))
                .transfer_lts(amount)
            );
    }

    pub fn withdraw(&mut self, amount:u128){
        let account = "rewarder_contract.testnet".to_string().try_into().unwrap();
        let account_lts= "light-token.testnet".to_string().try_into().unwrap();

        ext_ft::ext(account)
            .with_static_gas(Gas(5 * TGAS))
            .withdraw(env::signer_account_id().to_string(),amount)
            .and(
                ext_lts::ext(account_lts)
                .with_static_gas(Gas(2 * TGAS))
                .with_attached_deposit(1)
                .ft_transfer(env::signer_account_id().to_string(),(amount*100000000).to_string(),"".to_string())
            );
    }

}
