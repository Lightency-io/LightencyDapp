use near_sdk::{ext_contract};
use serde::{Serialize, Deserialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, env, near_bindgen, AccountId, Gas, Promise, PromiseError, PanicOnDefault};
use near_sdk::collections::Vector;




#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize,Deserialize)]
pub struct Data {
    amount:u128,
    time:u64,
}

#[ext_contract(ext_ft)]
pub trait Rewardpool {
    #[payable]
    fn pay(&mut self, amount: u128, to: String);
}

#[ext_contract(this_contract)]
trait Callbacks {
    fn get_data_callback(&self) -> Data;
}

#[ext_contract(ext_lightencypool)]
pub trait Lightencypool {
    fn get_data(&self, account:String) -> Vec<Data>;
    fn get_totalstaked(&self) -> u128;
}


// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Rewardercontract {
    redeemers:Vector<String>,
}

impl Default for Rewardercontract {
    fn default() -> Self {
        panic!("Contract is not initialized yet")
    }
}

// Implement the Rewardercontract structure
#[near_bindgen]
impl Rewardercontract {

    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            redeemers: Vector::new(b"a"),
        }
    }

    pub fn redeem(&mut self,account:String){
        self.redeemers.push(&account);
    }

    pub fn get_data_staker(&self, account:String) -> Promise{
        let contract = "lightencyrewardpool.testnet".to_string().try_into().unwrap();
        // Create a promise to call HelloNEAR.get_greeting()
        let promise = ext_lightencypool::ext(contract)
          .with_static_gas(Gas(5*1000000000000))
          .get_data(account);
        
        return promise.then( // Create a promise to callback query_greeting_callback
          Self::ext(env::current_account_id())
          .with_static_gas(Gas(5*1000000000000))
          .query_data_staker_callback()
        )
      }
    
        
      // Public - but only callable by env::current_account_id()
        pub fn query_data_staker_callback(&self, #[callback_result] call_result: Result<Vec<Data>, PromiseError>) -> Vec<Data> {
        // Check if the promise succeeded by calling the method outlined in external.rs
        if call_result.is_err() {
            panic!("There was an error contacting stakingpool contract");
        }
    
        // Return the greeting
        let data: Vec<Data> = call_result.unwrap();
        data
        }
 // ****** GETTER TOTAL STAKED*****//
  pub fn get_totalstaked(&self) -> Promise {
    let account = "lightencyrewardpool.testnet".to_string().try_into().unwrap();
    // Create a promise to call HelloNEAR.get_greeting()
    let promise = ext_lightencypool::ext(account)
      .with_static_gas(Gas(5*1000000000000))
      .get_totalstaked();
    
    return promise.then( // Create a promise to callback query_greeting_callback
      Self::ext(env::current_account_id())
      .with_static_gas(Gas(5*1000000000000))
      .query_totalstaked_callback()
    )
  }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn query_totalstaked_callback(&self, #[callback_result] call_result: Result<u128, PromiseError>) -> u128 {
    // Check if the promise succeeded by calling the method outlined in external.rs
    if call_result.is_err() {
        log!("There was an error contacting Hello NEAR");
        return 0;
    }

    // Return the greeting
    let totalstaked: u128 = call_result.unwrap();
    totalstaked
    }

    // pub fn get_totalstaked(&self){
    //     let promise = Lightencypool::get_totalstaked();
    //     return promise.then(
    //         Self::ext(env::current_account_id())
    //         .with_static_gas(Gas(5*1000000000000))
    //         .get_totalstaked_callback()
    //     )
    // }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn get_totalstaked_callback(&self, #[callback_result] call_result: Result<u128, PromiseError>) -> u128 {
    // Check if the promise succeeded by calling the method outlined in external.rs
    if call_result.is_err() {
        panic!("There was an error contacting staking Contract");
    }

    // Return the data
    let totalstaked: u128 = call_result.unwrap();
    totalstaked
  }

    pub fn pay(&self ,amount:u128 , to:String){
        let account = "lightencyrewardpool.testnet".to_string().try_into().unwrap();
        ext_ft::ext(account)
            .with_static_gas(Gas(5 * 1000000000000))
            .pay(amount,to);
        // let promise = ext_rewardpool::pay(
        //     "lightencyrewarder.testnet".to_string().try_into().unwrap(),
        //     0,
        //     Gas(50000000000000)
        // );
    }

    pub fn withdraw(&mut self){
        for i in 0..self.redeemers.len(){
            match self.redeemers.get(i){
                Some(r) => if r==env::signer_account_id().to_string() {
                    //let data2= self.query_data_staker_callback(self.get_data_staker(env::signer_account_id().to_string()));
                    //let rewards = self.calculaterewards(data2,data2.time);
                    self.pay(1,env::signer_account_id().to_string());
                    self.redeemers.swap_remove(i);
                },
                None => panic!("you are not included in redeemers list"),
            }

        }

    }

    pub fn calculaterewards(&self,amount:u128,time:u64)-> f64{
        //Reward to stakers= Total staked (t) X APY(t) 
        //APY(t)=Staking pool supply/total staked(t) X Yield parameter.
        let apy=0.1;
        let reward = amount as f64 * apy;
        return reward * 1000000000000000000000000.0;
    }
}


