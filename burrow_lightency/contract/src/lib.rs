use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ext_contract, Promise, PromiseError};
use near_sdk::{env, near_bindgen, Gas,AccountId};
use serde_json::Value;


pub const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(ext_burrow)]
pub trait Burrow {
    fn make_deposit_burrow(&mut self,amount: String,receiver_id: AccountId,msg: String) -> Promise;
    fn increase_colateral(&mut self, actions: Vec<Value>) -> Promise;
    fn make_burrow(&mut self, msg: String, receiver_id: AccountId) -> Promise;
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BurrowContract {

}

// Define the default, which automatically initializes the contract
impl Default for BurrowContract {
    fn default() -> Self {
        panic!("Contract is not initialized yet")
    }
}

// Implement the contract structure
// To be implemented in the front end
#[near_bindgen]
impl BurrowContract {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {

        }
    }
    // burrow function 
    pub fn burrow(&mut self,amount:String,account_id:AccountId,msg1:String,vec:Vec<Value>,msg2:String)-> Promise{
        let account1= "burrow_l.testnet".to_string().try_into().unwrap();
        let account2= "burrow_l.testnet".to_string().try_into().unwrap();
        let account3= "burrow_l.testnet".to_string().try_into().unwrap();
        // Create a promise to call make_deposit_burrow function 
        let promise = ext_burrow::ext(account1)
        .with_static_gas(Gas(46 * TGAS))
        .with_attached_deposit(1)
        .make_deposit_burrow(amount,account_id.clone(),msg1)
        .then(
            ext_burrow::ext(account2)
        .with_static_gas(Gas(46 * TGAS))
        .increase_colateral(vec)
        )
        .then(
            ext_burrow::ext(account3)
        .with_static_gas(Gas(70 * TGAS))
        .make_burrow(msg2,account_id)
        );
        promise
        
        // promise.then(
        //     ext_burrow::ext(account.clone())
        // .with_static_gas(Gas(1 * TGAS))
        // .increase_colateral(vec)
        // ).then(
        //     ext_burrow::ext(account.clone())
        // .with_static_gas(Gas(1 * TGAS))
        // .make_burrow(msg2,account_id)
        // )

        // return promise.then( // Create a promise to callback unstaking_callback
        //     Self::ext(env::current_account_id())
        //     .with_static_gas(Gas(5 * TGAS))
        //     .burrow1_callback(vec,msg2,account_id.clone())
        // )
    }

    // #[private] // Public - but only callable by env::current_account_id()
    // pub fn burrow1_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>,vec:Vec<Value>,msg2:String,account_id:AccountId) -> Promise {
    //     let account= "burrow_l.testnet".to_string().try_into().unwrap();
    //     // Check if the promise succeeded
    //     if call_result.is_err() {
    //     panic!("There was an error contacting the make_deposit_burrow function");
    //     }
    
    //     // Return the promise
    //     let promise = ext_burrow::ext(account)
    //     .with_static_gas(Gas(1 * TGAS))
    //     .increase_colateral(vec);

    //     return promise.then( // Create a promise to callback unstaking_callback
    //         Self::ext(env::current_account_id())
    //         .with_static_gas(Gas(3 * TGAS))
    //         .burrow2_callback(msg2,account_id)
    //     )
    // }

    // #[private] // Public - but only callable by env::current_account_id()
    // pub fn burrow2_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>,msg2:String,account_id:AccountId) -> Promise {
    //     let account= "burrow_l.testnet".to_string().try_into().unwrap();
    //     // Check if the promise succeeded
    //     if call_result.is_err() {
    //     panic!("There was an error contacting the increase_colateral function");
    //     }

    //     let promise = ext_burrow::ext(account)
    //     .with_static_gas(Gas(1 * TGAS))
    //     .make_burrow(msg2,account_id);
        
    //     return promise.then( // Create a promise to callback unstaking_callback
    //         Self::ext(env::current_account_id())
    //         .with_static_gas(Gas(1 * TGAS))
    //         .burrow3_callback()
    //     )
    // }

    // #[private] // Public - but only callable by env::current_account_id()
    // pub fn burrow3_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>){
    //     // Check if the promise succeeded
    //     if call_result.is_err() {
    //     panic!("There was an error contacting the make_burrow function");
    //     }
    // }
}



